//! # Windows 进程工具集
//!
//! 提供用于查询、列举和检查 Windows 进程的函数。
use crate::errors::Result;
use crate::utils::ProcessInfo;
use anyhow::bail;
use std::ffi::c_void;
use std::mem;
use windows_result::BOOL;
use windows::{
    core::PCWSTR,
    Win32::{
        Foundation::STILL_ACTIVE,
        Storage::FileSystem::{GetFileVersionInfoSizeW, GetFileVersionInfoW, VerQueryValueW, VS_FIXEDFILEINFO},
        System::{
            Diagnostics::ToolHelp::{
                CreateToolhelp32Snapshot, Process32FirstW, Process32NextW, PROCESSENTRY32W,
                TH32CS_SNAPPROCESS,
            },
            ProcessStatus::GetModuleFileNameExW,
            SystemInformation::{
                GetNativeSystemInfo, SYSTEM_INFO, PROCESSOR_ARCHITECTURE_AMD64,
                PROCESSOR_ARCHITECTURE_ARM64, PROCESSOR_ARCHITECTURE_IA64,
            },
            Threading::{
                GetExitCodeProcess, IsWow64Process, OpenProcess, PROCESS_QUERY_INFORMATION,
                PROCESS_QUERY_LIMITED_INFORMATION, PROCESS_VM_READ,
            },
        },
    },
};
use std::collections::HashSet;
use super::handle::Handle;

/// 列举系统中的所有进程，并根据过滤器和选项返回匹配的进程信息。
///
/// # 参数
///
/// * `filter` - 进程名称过滤器数组
/// * `main_process_only` - 是否只返回主进程，默认为 false
///   - `true`: 只返回主进程（没有父进程在结果集中的进程）
///   - `false`: 返回所有匹配的进程（主进程和子进程）
pub fn list_processes(filter: &[&str], main_process_only: bool) -> Result<Vec<ProcessInfo>> {
    let mut processes = Vec::new();
    let snapshot = Handle::new(unsafe { CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0)? })?;

    let mut process_entry = PROCESSENTRY32W::default();
    process_entry.dwSize = mem::size_of::<PROCESSENTRY32W>() as u32;

    if unsafe { Process32FirstW(*snapshot, &mut process_entry) }.is_err() {
        // 如果第一个就失败了，直接返回空列表或错误
        // 这里根据 ToolHelp 文档，如果进程列表为空，也会返回 ERROR_NO_MORE_FILES
        // 所以返回 Ok(vec![]) 是合理的。
        return Ok(processes);
    }

    loop {
        let process_name = unsafe { PCWSTR::from_raw(process_entry.szExeFile.as_ptr()).to_string()? };

        if filter.is_empty() || filter.iter().any(|name| name.eq_ignore_ascii_case(&process_name)) {
            let pid = process_entry.th32ProcessID;
            
            // 使用最少的权限打开进程，满足所有后续调用的需求
            // GetModuleFileNameExW: PROCESS_QUERY_LIMITED_INFORMATION | PROCESS_VM_READ
            // IsWow64Process: PROCESS_QUERY_LIMITED_INFORMATION
            let Ok(process_handle) = Handle::new(unsafe {
                OpenProcess(
                    PROCESS_QUERY_LIMITED_INFORMATION | PROCESS_VM_READ,
                    false,
                    pid,
                )?
            }) else {
                // 如果无法打开进程（例如权限不足），记录警告并跳过
                tracing::warn!("Failed to open process with PID {}: access denied or process terminated.", pid);
                if unsafe { Process32NextW(*snapshot, &mut process_entry) }.is_err() { break; }
                continue;
            };

            let path = get_process_exe_path_by_handle(&process_handle).unwrap_or_else(|e| {
                tracing::warn!("Failed to get path for PID {}: {}", pid, e);
                String::new()
            });
            let version = get_file_version_info(&path).unwrap_or_else(|e| {
                tracing::warn!("Failed to get version for path '{}': {}", path, e);
                String::new()
            });
            let is_64_bit = get_process_architecture_by_handle(&process_handle)
                .map(|arch| arch.is_64_bit())
                .unwrap_or_else(|e| {
                    tracing::warn!("Failed to get architecture for PID {}: {}", pid, e);
                    false
                });

            processes.push(ProcessInfo {
                parent_pid: process_entry.th32ParentProcessID,
                pid,
                name: process_name,
                path: Some(path),
                version: Some(version),
                is_64_bit,
                is_main_process: false,
            });
        }

        if unsafe { Process32NextW(*snapshot, &mut process_entry) }.is_err() {
            break;
        }
    }
    
    // ... is_main_process 逻辑保持不变 ...
    if !processes.is_empty() {
        let all_found_pids: HashSet<u32> = processes.iter().map(|p| p.pid).collect();

        for process in &mut processes {
            if !all_found_pids.contains(&process.parent_pid) {
                process.is_main_process = true;
            }
        }        
    }
    
    // 根据 main_process_only 参数过滤结果
    if main_process_only {
        processes.retain(|p| p.is_main_process);
    }
    
    Ok(processes)
}

/// 根据已打开的进程句柄获取其可执行文件的完整路径。
pub fn get_process_exe_path_by_handle(handle: &Handle) -> Result<String> {
    const MAX_PATH_LEN: usize = 1024;
    let mut exe_path_buffer: Vec<u16> = vec![0; MAX_PATH_LEN];

    let len = unsafe { GetModuleFileNameExW(Some(**handle), None, &mut exe_path_buffer) };

    if len == 0 {
        Err(windows::core::Error::from_win32().into())
    } else {
        Ok(String::from_utf16_lossy(&exe_path_buffer[..len as usize]))
    }
}

/// 根据 PID 获取其可执行文件的完整路径。
pub fn get_process_exe_path(pid: u32) -> Result<String> {

    let handle: Handle = Handle::new(unsafe {
        OpenProcess(PROCESS_QUERY_INFORMATION | PROCESS_VM_READ, false, pid)?
    })?;
    get_process_exe_path_by_handle(&handle)
}


/// 根据已打开的进程句柄判断一个进程的体系结构（32位或64位）。
pub fn get_process_architecture_by_handle(handle: &Handle) -> Result<ProcessArchitecture> {
    let mut is_wow64 = BOOL(0);
    unsafe { IsWow64Process(**handle, &mut is_wow64)? };

    if is_wow64.as_bool() {
        return Ok(ProcessArchitecture::Bit32);
    }

    let mut system_info = SYSTEM_INFO::default();
    unsafe { GetNativeSystemInfo(&mut system_info) };

    match unsafe { system_info.Anonymous.Anonymous.wProcessorArchitecture } {
        PROCESSOR_ARCHITECTURE_AMD64 | PROCESSOR_ARCHITECTURE_IA64 | PROCESSOR_ARCHITECTURE_ARM64 => {
            Ok(ProcessArchitecture::Bit64)
        }
        _ => Ok(ProcessArchitecture::Bit32),
    }
}

/// 判断一个进程的体系结构（32位或64位）。
pub fn get_process_architecture(pid: u32) -> Result<ProcessArchitecture> {
    let handle = Handle::new(unsafe { OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, false, pid)? })?;
    get_process_architecture_by_handle(&handle)
}


/// 获取文件的版本信息字符串（例如 "1.2.3.4"）。
pub fn get_file_version_info(exe_path: &str) -> Result<String> {
    if exe_path.is_empty() {
        return Ok(String::new());
    }
    
    let wide_path: Vec<u16> = exe_path.encode_utf16().chain(std::iter::once(0)).collect();
    let version_info_size = unsafe { GetFileVersionInfoSizeW(PCWSTR(wide_path.as_ptr()), None) };

    if version_info_size == 0 {
        return Err(windows::core::Error::from_win32().into());
    }

    let mut version_info_buffer = vec![0u8; version_info_size as usize];
    unsafe {
        GetFileVersionInfoW(
            PCWSTR(wide_path.as_ptr()),
            Some(0), // dwHandle, must be 0
            version_info_size,
            version_info_buffer.as_mut_ptr() as *mut c_void,
        )?;
    }

    let mut fixed_info_ptr: *mut c_void = std::ptr::null_mut();
    let mut len: u32 = 0;
    
    let sub_block: [u16; 2] = ['\\' as u16, 0];
    
    let success = unsafe {
        VerQueryValueW(
            version_info_buffer.as_ptr() as *const c_void,
            PCWSTR(sub_block.as_ptr()),
            &mut fixed_info_ptr,
            &mut len,
        )
    };

    if !success.as_bool() {
         return Err(windows::core::Error::from_win32().into());
    }

    if fixed_info_ptr.is_null() || len == 0 {
        bail!("VS_FIXEDFILEINFO not found in version data for '{}'", exe_path);
    }
    
    let fixed_info = unsafe { &*(fixed_info_ptr as *const VS_FIXEDFILEINFO) };
    if fixed_info.dwSignature != 0xFEEF04BD {
        bail!("Invalid VS_FIXEDFILEINFO signature for '{}'", exe_path);
    }

    let major = (fixed_info.dwFileVersionMS >> 16) & 0xffff;
    let minor = fixed_info.dwFileVersionMS & 0xffff;
    let build = (fixed_info.dwFileVersionLS >> 16) & 0xffff;
    let patch = fixed_info.dwFileVersionLS & 0xffff;
    
    Ok(format!("{}.{}.{}.{}", major, minor, build, patch))
}



/// 检查指定 PID 的进程是否仍在运行。
pub fn is_process_running(pid: u32) -> bool {
    if pid == 0 {
        return false;
    }
    
    // FIX: 使用分步 match 来代替 and_then，以解决不同 Error 类型的冲突。
    // 这种方式更清晰，也更容易调试。
    
    // 步骤 1: 尝试打开进程，获取原始句柄。
    let raw_handle = match unsafe { OpenProcess(PROCESS_QUERY_INFORMATION, false, pid) } {
        Ok(h) => h,
        Err(_) => return false, // 如果 OpenProcess 失败，进程不可访问，视为 "不在运行"。
    };

    // 步骤 2: 将原始句柄包装到我们的 RAII `Handle` 中。
    // 这一步是必要的，以确保句柄在任何情况下都会被关闭。
    // `Handle::new` 也会检查句柄是否有效 (INVALID_HANDLE_VALUE)。
    let process_handle = match Handle::new(raw_handle) {
        Ok(p) => p,
        Err(_) => return false, // 如果句柄无效，也视为 "不在运行"。
    };

    // 步骤 3: 获取进程退出码。
    let mut exit_code: u32 = 0;
    if unsafe { GetExitCodeProcess(*process_handle, &mut exit_code) }.is_ok() {
        exit_code == STILL_ACTIVE.0 as u32
    } else {
        false // 如果 GetExitCodeProcess 失败，无法确定状态，安全起见返回 false。
    }
}

/// 定义一个枚举来清晰地表示进程架构
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProcessArchitecture {
    Bit32,
    Bit64,
}

impl ProcessArchitecture {
    pub fn is_64_bit(&self) -> bool {
        *self == ProcessArchitecture::Bit64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_processes_with_options_signature() {
        // 测试新函数的签名和基本调用
        // 使用一个不太可能存在的进程名来避免权限问题
        let result1 = list_processes(&["nonexistent_process.exe"], false);
        let result2 = list_processes(&["nonexistent_process.exe"], true);
        
        // 两个调用都应该成功（即使返回空列表）
        match (result1, result2) {
            (Ok(all), Ok(main)) => {
                println!("✅ list_processes_with_options 函数签名正确");
                println!("所有进程: {} 个, 主进程: {} 个", all.len(), main.len());
                
                // 主进程数量应该小于等于总进程数量
                assert!(main.len() <= all.len(), "主进程数量应该小于等于总进程数量");
                
                // 所有返回的主进程都应该标记为主进程
                for process in &main {
                    assert!(process.is_main_process, "返回的进程应该是主进程");
                }
            }
            (Err(e1), _) => {
                println!("⚠️ list_processes_with_options(false) 失败: {}", e1);
            }
            (_, Err(e2)) => {
                println!("⚠️ list_processes_with_options(true) 失败: {}", e2);
            }
        }
    }

    #[test]
    fn test_process_architecture() {
        let arch_32 = ProcessArchitecture::Bit32;
        let arch_64 = ProcessArchitecture::Bit64;
        
        assert!(!arch_32.is_64_bit());
        assert!(arch_64.is_64_bit());
    }

}