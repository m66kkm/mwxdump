//! MWXDump CLI Library
//! 
//! 这是 MWXDump CLI 应用程序的库部分，提供 CLI 特定的功能。

// 重新导出核心库
pub use mwxdump_core::*;

// CLI 特定模块
pub mod app;
pub mod cli;
pub mod config;

// 为 HTTP 响应添加错误转换
use axum::response::IntoResponse;
use axum::http::StatusCode;
use axum::Json;
use serde_json::json;

/// HTTP 错误包装器
#[derive(Debug)]
pub struct HttpError(pub mwxdump_core::errors::MwxDumpError);

impl From<mwxdump_core::errors::MwxDumpError> for HttpError {
    fn from(err: mwxdump_core::errors::MwxDumpError) -> Self {
        Self(err)
    }
}

impl IntoResponse for HttpError {
    fn into_response(self) -> axum::response::Response {
        let (status, error_message) = match self.0 {
            mwxdump_core::errors::MwxDumpError::Http(ref http_err) => {
                match http_err {
                    mwxdump_core::errors::HttpError::ResourceNotFound { .. } => {
                        (StatusCode::NOT_FOUND, self.0.to_string())
                    }
                    mwxdump_core::errors::HttpError::AuthenticationFailed => {
                        (StatusCode::UNAUTHORIZED, self.0.to_string())
                    }
                    _ => (StatusCode::INTERNAL_SERVER_ERROR, self.0.to_string())
                }
            }
            mwxdump_core::errors::MwxDumpError::Database(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, "数据库错误".to_string())
            }
            _ => (StatusCode::INTERNAL_SERVER_ERROR, "内部服务器错误".to_string()),
        };
        
        let body = Json(json!({
            "error": error_message,
            "code": status.as_u16()
        }));
        
        (status, body).into_response()
    }
}

/// CLI 应用程序版本信息
pub const CLI_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const CLI_NAME: &str = env!("CARGO_PKG_NAME");

/// 初始化 CLI 应用程序
pub fn init_cli() -> mwxdump_core::Result<()> {
    // 初始化核心库
    mwxdump_core::init()?;
    
    // 初始化日志
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_cli() {
        assert!(init_cli().is_ok());
    }

    #[test]
    fn test_version() {
        assert!(!CLI_VERSION.is_empty());
        assert!(!CLI_NAME.is_empty());
    }
}