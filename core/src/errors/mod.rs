//! 错误处理模块
//! 
//! 定义了应用中所有可能的错误类型，使用thiserror简化错误定义

use thiserror::Error;

pub type Result<T> = anyhow::Result<T>;

/// 应用主要错误类型
#[derive(Error, Debug)] // Clone, PartialEq, Eq are useful for testing
pub enum MwxDumpError {
    #[error("配置错误: {0}")]
    Config(#[from] ConfigError),
    
    #[error("数据库错误: {0}")]
    Database(#[from] DatabaseError),
    
    #[error("微信相关错误: {0}")]
    WeChat(#[from] WeChatError),
    
    #[error("HTTP服务错误: {0}")]
    Http(#[from] HttpError),
    
    #[error("MCP协议错误: {0}")]
    Mcp(#[from] McpError),
    
    #[error("UI错误: {0}")]
    Ui(#[from] UiError),
    
    #[error("IO错误: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("序列化错误: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("系统错误: '{0}'")]
    System(#[from] SystemError),
  
    #[error("无效或无法解析的版本字符串: '{0}'")]
    InvalidVersion(String),
    
    #[error("其他错误: {0}")]
    Other(#[from] anyhow::Error),
}

/// 配置相关错误
#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("配置文件不存在: {path}")]
    FileNotFound { path: String },
    
    #[error("配置文件格式错误: {0}")]
    ParseError(String),
    
    #[error("配置项缺失: {key}")]
    MissingKey { key: String },
    
    #[error("配置项值无效: {key} = {value}")]
    InvalidValue { key: String, value: String },
}

#[derive(Error, Debug)]
pub enum SystemError {

    #[error("模块信息获取失败: {value} - pid: {pid}")]
    ModuleInfoMissing{ value: String, pid: u32 },
 
    #[error("未知系统错误: {value}")]
    UnknownError { value: String },
    
    #[error("进程路径缺失")]
    MissingPath,
}


/// 数据库相关错误
#[derive(Error, Debug)]
pub enum DatabaseError {
    #[error("数据库连接失败: {0}")]
    ConnectionFailed(String),
    
    #[error("SQL执行错误: {0}")]
    SqlError(#[from] sqlx::Error),
    
    #[error("数据库文件不存在: {path}")]
    FileNotFound { path: String },
    
    #[error("数据库版本不支持: {version}")]
    UnsupportedVersion { version: String },
    
    #[error("数据迁移失败: {0}")]
    MigrationFailed(String),
}

/// 微信相关错误
#[derive(Error, Debug)]
pub enum WeChatError {
    #[error("微信进程未找到")]
    ProcessNotFound,
    
    #[error("密钥提取失败: {0}")]
    KeyExtractionFailed(String),
    
    #[error("数据解密失败: {0}")]
    DecryptionFailed(String),
    
    #[error("不支持的微信版本: {version}， 请升级到4.0+版本")]
    UnsupportedVersion { version: String },
    
    #[error("权限不足: {0}")]
    PermissionDenied(String),
    
    #[error("数据文件损坏: {path}")]
    CorruptedFile { path: String },
}

/// HTTP服务相关错误
#[derive(Error, Debug)]
pub enum HttpError {
    #[error("服务器启动失败: {0}")]
    ServerStartFailed(String),
    
    #[error("端口被占用: {port}")]
    PortInUse { port: u16 },
    
    #[error("请求处理失败: {0}")]
    RequestFailed(String),
    
    #[error("认证失败")]
    AuthenticationFailed,
    
    #[error("资源未找到: {resource}")]
    ResourceNotFound { resource: String },
}

/// MCP协议相关错误
#[derive(Error, Debug)]
pub enum McpError {
    #[error("协议解析错误: {0}")]
    ProtocolError(String),
    
    #[error("会话不存在: {session_id}")]
    SessionNotFound { session_id: String },
    
    #[error("工具执行失败: {tool} - {error}")]
    ToolExecutionFailed { tool: String, error: String },
    
    #[error("资源访问失败: {resource}")]
    ResourceAccessFailed { resource: String },
}

/// UI相关错误
#[derive(Error, Debug)]
pub enum UiError {
    #[error("终端初始化失败: {0}")]
    TerminalInitFailed(String),
    
    #[error("渲染错误: {0}")]
    RenderError(String),
    
    #[error("事件处理错误: {0}")]
    EventHandlingError(String),
}

// HTTP 响应转换将在 CLI 项目中单独实现
// 这里只保留核心错误定义

#[cfg(target_os = "windows")]
impl From<windows::core::Error> for MwxDumpError {
    fn from(err: windows::core::Error) -> Self {
        MwxDumpError::WeChat(WeChatError::ProcessNotFound)
    }
}