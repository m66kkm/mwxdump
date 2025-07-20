# MwXdump 项目整合实施计划

## 概述

本文档详细描述了将 `mwxdump-rs` 和 `mwxdump-ui` 两个项目整合的具体实施步骤和技术方案。

## 整合目标

1. **代码复用**: 创建共享核心库，避免重复实现
2. **架构统一**: 建立清晰的模块边界和依赖关系
3. **开发效率**: 加速UI项目开发，保持功能同步
4. **维护性**: 统一错误处理、日志和配置管理

## 实施阶段

### Phase 1: 核心库提取 (预计 1-2 天)

#### 1.1 创建 Workspace 结构

```toml
# Cargo.toml (根目录)
[workspace]
members = [
    "mwxdump-core",
    "mwxdump-rs", 
    "mwxdump-ui/src-tauri"
]

[workspace.dependencies]
# 共享依赖版本管理
tokio = { version = "1.46", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
thiserror = "2.0.12"
anyhow = "1.0"
chrono = { version = "0.4", features = ["serde"] }
uuid = { version = "1.17", features = ["v4", "serde"] }
tracing = "0.1"
```

#### 1.2 创建核心共享库

**目录结构**:
```
mwxdump-core/
├── Cargo.toml
├── src/
│   ├── lib.rs
│   ├── errors/
│   │   └── mod.rs
│   ├── models/
│   │   ├── mod.rs
│   │   ├── message.rs
│   │   ├── contact.rs
│   │   ├── chatroom.rs
│   │   └── session.rs
│   ├── wechat/
│   │   ├── mod.rs
│   │   ├── decrypt/
│   │   ├── key/
│   │   ├── process/
│   │   └── wechat_version.rs
│   └── utils/
│       ├── mod.rs
│       └── windows/
└── README.md
```

**核心库 Cargo.toml**:
```toml
[package]
name = "mwxdump-core"
version = "0.1.0"
edition = "2021"
description = "MwXdump 核心功能库"

[dependencies]
tokio = { workspace = true }
serde = { workspace = true }
serde_json = { workspace = true }
thiserror = { workspace = true }
anyhow = { workspace = true }
chrono = { workspace = true }
uuid = { workspace = true }
tracing = { workspace = true }

# 加密相关
aes = "0.8"
cbc = "0.1"
hmac = "0.12"
sha1 = "0.10"
sha2 = "0.10"
pbkdf2 = "0.12"
hex = "0.4"
zeroize = "1.8"
byteorder = "1.5"

# 压缩
lz4 = "1.28"
flate2 = "1.1"

# 系统信息
sysinfo = "^0.36"

[target.'cfg(windows)'.dependencies]
windows = { version = "0.61", features = [
    "Win32_System_ProcessStatus",
    "Win32_System_Diagnostics_ToolHelp",
    "Win32_System_Diagnostics_Debug",
    "Win32_System_Threading",
    "Win32_Foundation",
    "Win32_System_Memory",
    "Win32_Security",
    "Win32_System_SystemInformation",
    "Win32_System_Registry",
    "Win32_Storage_FileSystem",
    "Win32_System_WindowsProgramming",
]}
```

#### 1.3 模块迁移清单

**需要迁移的模块**:

1. **错误处理** (`src/errors/`)
   - `MwxDumpError` 枚举
   - `Result<T>` 类型别名
   - 错误转换实现

2. **数据模型** (`src/models/`)
   - `Message` 结构体
   - `Contact` 结构体  
   - `ChatRoom` 结构体
   - `Session` 结构体

3. **微信功能** (`src/wechat/`)
   - `decrypt/` 解密模块
   - `key/` 密钥提取模块
   - `process/` 进程检测模块
   - `wechat_version.rs` 版本管理

4. **工具函数** (`src/utils/`)
   - `windows/` Windows API 封装
   - 通用工具函数

### Phase 2: CLI 项目重构 (预计 1 天)

#### 2.1 更新 CLI 项目依赖

```toml
# mwxdump-rs/Cargo.toml
[package]
name = "mwxdump-rs"
version = "0.1.0"
edition = "2021"

[[bin]]
name = "mwx-cli"
path = "src/main.rs"

[dependencies]
mwxdump-core = { path = "../mwxdump-core" }

# CLI 特有依赖
clap = { version = "4.5", features = ["derive", "env"] }
config = "^0.15"
toml = "^0.9"

# Web 服务 (可选)
axum = { version = "0.8", features = ["multipart"] }
tower = { version = "0.5", features = ["full"] }
tower-http = { version = "0.6.6", features = ["cors", "fs", "trace"] }

# TUI (可选)
ratatui = "0.29.0"
crossterm = "0.29"

# 其他
tracing-subscriber = { version = "0.3", features = ["env-filter", "json"] }
tracing-appender = "0.2"
```

#### 2.2 重构 CLI 代码结构

**保留的模块**:
- `src/main.rs` - CLI 入口点
- `src/cli/` - 命令行接口
- `src/config/` - 配置管理
- `src/http/` - Web 服务 (可选)
- `src/ui/` - TUI 界面 (可选)

**更新导入**:
```rust
// src/main.rs
use mwxdump_core::{Result, MwxDumpError};
use mwxdump_core::models::{Message, Contact, ChatRoom, Session};
use mwxdump_core::wechat::{decrypt, key, process};
```

### Phase 3: UI 项目整合 (预计 2-3 天)

#### 3.1 更新 Tauri 后端依赖

```toml
# mwxdump-ui/src-tauri/Cargo.toml
[package]
name = "mwxdump-ui"
version = "0.1.0"
edition = "2021"

[lib]
name = "mwxdump_ui_lib"
c
crate-type = ["staticlib", "cdylib", "rlib"]

[dependencies]
mwxdump-core = { path = "../../mwxdump-core" }

# Tauri 相关
tauri = { version = "2", features = [] }
tauri-plugin-opener = "2"
serde = { workspace = true }
serde_json = { workspace = true }

# 异步运行时
tokio = { workspace = true }

[build-dependencies]
tauri-build = { version = "2", features = [] }
```

#### 3.2 实现 Tauri 命令接口

**核心命令实现**:
```rust
// mwxdump-ui/src-tauri/src/lib.rs
use mwxdump_core::{Result, MwxDumpError};
use mwxdump_core::models::{Message, Contact, ChatRoom, Session};
use mwxdump_core::wechat::{decrypt, key, process};

#[tauri::command]
async fn detect_wechat_process() -> Result<Vec<process::WeChatProcessInfo>, String> {
    process::detect_processes()
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn extract_wechat_key(pid: u32) -> Result<String, String> {
    key::extract_key(pid)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn decrypt_database(
    input_path: String,
    output_path: String,
    key: String
) -> Result<(), String> {
    decrypt::decrypt_file(&input_path, &output_path, &key)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_contacts() -> Result<Vec<Contact>, String> {
    // 实现联系人获取逻辑
    Ok(vec![])
}

#[tauri::command]
async fn get_messages(contact_id: String) -> Result<Vec<Message>, String> {
    // 实现消息获取逻辑
    Ok(vec![])
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            detect_wechat_process,
            extract_wechat_key,
            decrypt_database,
            get_contacts,
            get_messages
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

#### 3.3 前端接口定义

**TypeScript 类型定义**:
```typescript
// mwxdump-ui/src/types/index.ts
export interface WeChatProcessInfo {
  pid: number;
  name: string;
  version: string;
  path: string;
}

export interface Contact {
  id: string;
  name: string;
  avatar?: string;
  type: 'user' | 'group';
}

export interface Message {
  id: string;
  contact_id: string;
  content: string;
  timestamp: number;
  type: 'text' | 'image' | 'file' | 'system';
  sender?: string;
}

export interface ChatRoom {
  id: string;
  name: string;
  members: Contact[];
  created_at: number;
}
```

**API 服务封装**:
```typescript
// mwxdump-ui/src/services/api.ts
import { invoke } from '@tauri-apps/api/core';
import { WeChatProcessInfo, Contact, Message } from '../types';

export class MwxdumpAPI {
  static async detectWeChatProcess(): Promise<WeChatProcessInfo[]> {
    return await invoke('detect_wechat_process');
  }

  static async extractWeChatKey(pid: number): Promise<string> {
    return await invoke('extract_wechat_key', { pid });
  }

  static async decryptDatabase(
    inputPath: string,
    outputPath: string,
    key: string
  ): Promise<void> {
    return await invoke('decrypt_database', {
      inputPath,
      outputPath,
      key
    });
  }

  static async getContacts(): Promise<Contact[]> {
    return await invoke('get_contacts');
  }

  static async getMessages(contactId: string): Promise<Message[]> {
    return await invoke('get_messages', { contactId });
  }
}
```

### Phase 4: 测试和验证 (预计 1 天)

#### 4.1 单元测试

**核心库测试**:
```rust
// mwxdump-core/tests/integration_tests.rs
#[cfg(test)]
mod tests {
    use mwxdump_core::*;

    #[tokio::test]
    async fn test_process_detection() {
        // 测试进程检测功能
    }

    #[tokio::test]
    async fn test_key_extraction() {
        // 测试密钥提取功能
    }

    #[tokio::test]
    async fn test_decryption() {
        // 测试解密功能
    }
}
```

#### 4.2 集成测试

**CLI 测试**:
```bash
# 测试 CLI 命令
cargo run --bin mwx-cli -- process
cargo run --bin mwx-cli -- key
cargo run --bin mwx-cli -- decrypt --input test.db --output decrypted.db
```

**UI 测试**:
```bash
# 启动 Tauri 开发服务器
cd mwxdump-ui
npm run tauri dev
```

## 风险评估和缓解策略

### 技术风险

1. **依赖冲突**
   - 风险: 不同版本的依赖可能导致编译错误
   - 缓解: 使用 workspace 统一管理依赖版本

2. **平台兼容性**
   - 风险: Windows API 在不同平台的兼容性问题
   - 缓解: 使用条件编译和特征门控

3. **性能影响**
   - 风险: 共享库可能增加编译时间
   - 缓解: 优化模块划分，减少不必要的依赖

### 实施风险

1. **代码迁移复杂性**
   - 风险: 模块间依赖关系复杂，迁移困难
   - 缓解: 分步骤迁移，保持向后兼容

2. **功能回归**
   - 风险: 重构过程中可能引入新的 bug
   - 缓解: 完善的测试覆盖，逐步验证

## 时间计划

| 阶段 | 任务 | 预计时间 | 里程碑 |
|------|------|----------|--------|
| Phase 1 | 核心库提取 | 1-2 天 | 共享库可编译 |
| Phase 2 | CLI 重构 | 1 天 | CLI 功能正常 |
| Phase 3 | UI 整合 | 2-3 天 | 基础 UI 功能 |
| Phase 4 | 测试验证 | 1 天 | 功能验证通过 |

**
总时间**: 5-7 天

## 成功标准

### 技术标准
1. ✅ 所有项目能够正常编译
2. ✅ CLI 功能保持完整
3. ✅ UI 能够调用核心功能
4. ✅ 测试覆盖率 > 80%
5. ✅ 性能无明显下降

### 功能标准
1. ✅ 微信进程检测正常
2. ✅ 密钥提取功能正常
3. ✅ 数据解密功能正常
4. ✅ UI 基础界面完成
5. ✅ 前后端通信正常

## 后续优化

### 短期优化 (1-2 周)
1. **UI 界面完善**
   - 实现完整的用户界面
   - 添加进度指示和错误处理
   - 优化用户体验

2. **功能扩展**
   - 批量处理功能
   - 数据导出功能
   - 配置管理界面

### 中期优化 (1 个月)
1. **性能优化**
   - 异步处理优化
   - 内存使用优化
   - 大文件处理优化

2. **稳定性提升**
   - 错误恢复机制
   - 日志系统完善
   - 异常处理增强

### 长期规划 (3 个月)
1. **跨平台支持**
   - macOS 平台适配
   - Linux 平台支持 (如适用)

2. **功能扩展**
   - 云端同步功能
   - 数据分析功能
   - 插件系统

## 附录

### A. 依赖版本对照表

| 依赖 | mwxdump-rs | mwxdump-ui | 建议版本 |
|------|------------|------------|----------|
| tokio | 1.46 | - | 1.46 |
| serde | 1.0 | 1.0 | 1.0 |
| tauri | - | 2.0 | 2.0 |
| clap | 4.5 | - | 4.5 |

### B. 文件迁移清单

**从 mwxdump-rs 迁移到 mwxdump-core**:
- `src/errors/mod.rs` → `mwxdump-core/src/errors/mod.rs`
- `src/models/*.rs` → `mwxdump-core/src/models/*.rs`
- `src/wechat/**/*.rs` → `mwxdump-core/src/wechat/**/*.rs`
- `src/utils/**/*.rs` → `mwxdump-core/src/utils/**/*.rs`

### C. API 接口规范

**Tauri 命令命名规范**:
- 使用 snake_case 命名
- 动词开头，描述具体操作
- 返回 Result<T, String> 类型

**错误处理规范**:
- 使用统一的错误类型
- 提供详细的错误信息
- 支持错误链追踪

---

**文档版本**: 1.0  
**创建日期**: 2025-07-20  
**最后更新**: 2025-07-20  
**审核状态**: 待审核