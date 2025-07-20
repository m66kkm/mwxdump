//! MWXDump Core Library
//! 
//! 这是一个共享的核心库，提供微信数据处理的核心功能，
//! 可以被 CLI 和 GUI 应用程序共同使用。

pub mod errors;
pub mod logs;
pub mod models;
pub mod wechat;
pub mod utils;

// 重新导出常用类型
pub use errors::{MwxDumpError as Error, Result};
pub use models::{Contact, Message, ChatRoom, Session};
pub use wechat::WeChatVersion;
pub use wechat::process::{WechatProcessInfo, ProcessDetector};

/// 库版本信息
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const NAME: &str = env!("CARGO_PKG_NAME");

/// 初始化库
pub fn init() -> Result<()> {
    // 初始化日志等基础设施
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init() {
        assert!(init().is_ok());
    }

    #[test]
    fn test_version() {
        assert!(!VERSION.is_empty());
        assert!(!NAME.is_empty());
    }
}