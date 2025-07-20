# 日志系统架构设计

## 文档信息
- **版本**: v1.0
- **创建日期**: 2025-01-20
- **文档类型**: Architecture Design
- **范围**: 日志系统重构与优化

## 概述

本文档描述了 MWXDump 项目中日志系统的重构设计，包括时间格式化函数的性能优化、模块化架构设计以及统一的日志管理方案。

## 设计目标

### 1. 性能优化
- 消除重复的系统时间调用
- 减少字符串分配和内存开销
- 提供高效的时间缓存机制

### 2. 架构改进
- 模块化设计，提高代码复用性
- 统一的日志接口，简化使用方式
- 灵活的配置系统，适应不同环境需求

### 3. 可维护性
- 清晰的模块边界和职责分离
- 完善的错误处理和降级机制
- 详细的文档和使用示例

## 架构设计

### 模块结构
```
core/src/logs/
├── mod.rs              # 模块导出和公共接口
├── time_format.rs      # 时间格式化器实现
├── config.rs           # 日志配置系统
├── formatter.rs        # 自定义事件格式化器
└── init.rs            # 日志系统初始化
```

### 核心组件

#### 1. 时间格式化器 (`time_format.rs`)

**OptimizedTimeFormat**
- 使用 `OnceLock<String>` 实现时间格式缓存
- 通过 `AtomicU64` 跟踪最后更新的秒数
- 避免重复的系统调用和字符串格式化

**ConfigurableTimeFormat**
- 支持多种时间精度（秒、毫秒、微秒、纳秒）
- 可配置的时间格式字符串
- 灵活的显示选项

#### 2. 配置系统 (`config.rs`)

**LogConfig 结构**
```rust
pub struct LogConfig {
    pub level: LogLevel,
    pub format: LogFormat,
    pub time_precision: TimePrecision,
    pub output: LogOutput,
    pub show_target: bool,
    pub show_thread_id: bool,
    pub show_line_number: bool,
}
```

**预设配置**
- `console()`: 控制台输出配置
- `debug()`: 调试模式配置
- `file()`: 文件输出配置
- `production()`: 生产环境配置

#### 3. 事件格式化器 (`formatter.rs`)

**CustomEventFormatter**
- 实现 `tracing_subscriber::fmt::FormatEvent` trait
- 支持自定义时间格式化器
- 可配置的字段显示选项

#### 4. 初始化系统 (`init.rs`)

**初始化函数**
- `init_with_config()`: 使用配置初始化
- `init_console()`: 快速控制台初始化
- `init_file()`: 文件输出初始化

## 性能优化策略

### 1. 时间缓存机制

**原始实现问题**
```rust
// 每次调用都会产生系统调用
fn format_time() -> String {
    chrono::Local::now().format("%y/%m/%d %H:%M:%S").to_string()
}
```

**优化后实现**
```rust
pub struct OptimizedTimeFormat {
    cached_format: OnceLock<String>,
    last_second: AtomicU64,
}

impl OptimizedTimeFormat {
    pub fn format_time(&self) -> String {
        let now = chrono::Local::now();
        let current_second = now.timestamp() as u64;
        
        // 只有当秒数变化时才重新格式化
        if self.last_second.load(Ordering::Relaxed) != current_second {
            let formatted = now.format("%y/%m/%d %H:%M:%S").to_string();
            self.cached_format.set(formatted.clone()).ok();
            self.last_second.store(current_second, Ordering::Relaxed);
        }
        
        self.cached_format.get().cloned().unwrap_or_default()
    }
}
```

### 2. 内存优化
- 使用 `OnceLock` 避免重复分配
- 原子操作减少锁竞争
- 字符串复用减少 GC 压力

### 3. 并发安全
- `AtomicU64` 提供无锁的时间戳比较
- `OnceLock` 确保线程安全的缓存更新
- 支持多线程环境下的高并发访问

## 错误处理设计

### 错误类型定义
```rust
#[derive(Debug, thiserror::Error)]
pub enum LogError {
    #[error("Invalid log level: {0}")]
    InvalidLevel(String),
    
    #[error("Invalid time precision: {0}")]
    InvalidPrecision(String),
    
    #[error("Configuration error: {0}")]
    ConfigError(String),
    
    #[error("Initialization failed: {0}")]
    InitError(String),
}
```

### 降级策略
- 时间格式化失败时使用默认格式
- 配置解析失败时使用默认配置
- 输出目标不可用时回退到控制台

## 集成方案

### CLI 集成
```rust
// cli/src/main.rs
use mwxdump_core::logs::{init_console, OptimizedTimeFormat};

fn main() -> Result<()> {
    // 使用新的日志系统
    init_console()?;
    
    // 使用优化的时间格式化器
    let time_formatter = OptimizedTimeFormat::new();
    tracing::info!("Application started at {}", time_formatter.format_time());
    
    Ok(())
}
```

### GUI 集成
```rust
// 未来的 GUI 应用可以使用相同的日志系统
use mwxdump_core::logs::{LogConfig, init_with_config};

fn init_gui_logging() -> Result<()> {
    let config = LogConfig::console()
        .with_level(LogLevel::Info)
        .with_show_thread_id(true);
    
    init_with_config(config)
}
```

## 向后兼容性

### 接口保持
- 保持原有的 `%y/%m/%d %H:%M:%S` 时间格式
- 提供简单的迁移路径
- 不破坏现有的日志输出格式

### 迁移指南
1. 替换原有的 `format_time()` 调用
2. 使用 `init_console()` 初始化日志系统
3. 可选：配置更高级的日志选项

## 测试策略

### 单元测试
- 时间格式化器的缓存机制测试
- 配置系统的各种组合测试
- 错误处理和边界情况测试