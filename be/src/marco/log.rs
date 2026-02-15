/// 日志工具宏
///
/// 注意：request_id 会自动包含在日志中，不需要手动添加
/// tower-http 中间件已将 request_id 添加到 http_request span 中
///
/// 使用示例：
/// ```rust
/// use crate::log_info;
/// log_info!("用户登录", user_id = 123);
/// log_error!("数据库连接失败", error = %e);
/// ```

/// 简化通用日志宏（info 级别）
#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => {
        tracing::info!($($arg)*)
    };
}

/// 简化 info 日志宏
#[macro_export]
macro_rules! log_info {
    ($($arg:tt)*) => {
        tracing::info!($($arg)*)
    };
}

/// 简化 error 日志宏
#[macro_export]
macro_rules! log_error {
    ($($arg:tt)*) => {
        tracing::error!($($arg)*)
    };
}

/// 简化 warn 日志宏
#[macro_export]
macro_rules! log_warn {
    ($($arg:tt)*) => {
        tracing::warn!($($arg)*)
    };
}

/// 简化 debug 日志宏
#[macro_export]
macro_rules! log_debug {
    ($($arg:tt)*) => {
        tracing::debug!($($arg)*)
    };
}
