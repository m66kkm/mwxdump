use clap::Parser;
use tracing::{info, error};
use mwxdump_core::errors::Result;
mod app;
mod cli;
mod config;

use cli::Cli;

#[tokio::main]
async fn main() -> Result<()> {
    // 解析命令行参数
    let cli = Cli::parse();
    
    // 创建执行上下文以确定最终的日志级别
    let context = match cli::context::ExecutionContext::new(cli.config.clone(), cli.log_level.clone()) {
        Ok(ctx) => ctx,
        Err(e) => {
            eprintln!("创建执行上下文失败: {}", e);
            std::process::exit(1);
        }
    };
    
    // 根据配置初始化日志系统
    init_tracing(&context)?;
    
    info!("MwXdump 启动，日志级别: {}", context.log_level());
    
    // 执行命令，传递已创建的上下文
    if let Err(e) = cli.execute_with_context(context).await {
        error!("执行失败: {}", e);
        
        // 打印更详细的错误信息到控制台
        eprintln!("\n执行失败: {}", e);
        
        // 将错误转换为anyhow::Error以便获取更多信息
        let err_any = anyhow::anyhow!("{}", e);
        
        // 检查错误源
        if let Some(source) = err_any.source() {
            eprintln!("错误原因: {}", source);
        }
        
        // 如果是微信相关错误，提供更详细的错误信息和解决方案
        if e.to_string().contains("微信进程未找到") {
            eprintln!("详细信息: 未找到微信进程，请确保微信正在运行");
        } else if e.to_string().contains("密钥提取失败") {
            eprintln!("详细信息: 密钥提取失败，可能原因:");
            eprintln!("  - 权限不足，请尝试以管理员身份运行");
            eprintln!("  - 微信版本不受支持");
            eprintln!("  - 内存搜索算法需要优化");
        } else if e.to_string().contains("权限不足") {
            eprintln!("详细信息: 权限不足，请尝试以管理员身份运行");
        }
        
        std::process::exit(1);
    }
    
    Ok(())
}

fn init_tracing(context: &cli::context::ExecutionContext) -> Result<()> {
    use mwxdump_core::logs::{LogConfig, LogLevel, LogOutput, init_tracing_with_config};
    
    // 根据执行上下文创建日志配置
    let log_level = match context.log_level().to_lowercase().as_str() {
        "error" => LogLevel::Error,
        "warn" | "warning" => LogLevel::Warn,
        "info" => LogLevel::Info,
        "debug" => LogLevel::Debug,
        "trace" => LogLevel::Trace,
        _ => LogLevel::Info,
    };
    
    let logging_config = context.logging_config();
    
    // 根据日志配置决定输出方式
    let output = match (&logging_config.console, &logging_config.file) {
        (true, Some(log_file_path)) => {
            // 同时输出到控制台和文件 - 简化处理，优先使用文件
            LogOutput::File(log_file_path.to_string_lossy().to_string())
        }
        (true, None) => LogOutput::Stdout,
        (false, Some(log_file_path)) => {
            LogOutput::File(log_file_path.to_string_lossy().to_string())
        }
        (false, None) => LogOutput::Stdout,
    };
    
    let config = LogConfig {
        level: log_level,
        output,
        show_target: false,
        show_thread_id: false,
        show_file_line: false,
        time_format: "%y/%m/%d %H:%M:%S".to_string(), // 保持与原代码兼容
        enable_colors: true,
        enable_time_cache: true,
        max_file_size: None,
        max_files: None,
    };
    
    // 使用 core 模块的日志初始化功能 - 只调用一次
    init_tracing_with_config(&config)?;
    
    Ok(())
}