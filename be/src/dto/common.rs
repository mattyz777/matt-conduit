//! 通用 DTO 定义
//! 统一的 API 响应结构
//! Option 序列化为 JSON 时 None 变成 null
use serde::Serialize;
use axum::http::StatusCode;

/// 统一的 API 响应结构
///
/// # 示例
///
/// ## 成功有数据
/// ```json
/// {
///   "code": 0,
///   "message": "success",
///   "data": { "id": 1, "username": "admin" }
/// }
/// ```
///
/// ## 成功无数据
/// ```json
/// {
///   "code": 0,
///   "message": "删除成功",
///   "data": null
/// }
/// ```
///
/// ## 错误响应
/// ```json
/// {
///   "code": 400,
///   "message": "用户名已存在",
///   "data": null
/// }
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct ApiResponse<T> {
    pub code: u16,
    pub message: String,
    pub data: Option<T>,
}

impl<T> ApiResponse<T> {
    /// 成功响应（有数据）
    pub fn ok(data: T) -> Self {
        Self {
            code: 0,
            message: "success".to_string(),
            data: Some(data),
        }
    }

    /// 成功响应（有数据 + 自定义消息）
    pub fn ok_with_message(data: T, message: &str) -> Self {
        Self {
            code: 0,
            message: message.to_string(),
            data: Some(data),
        }
    }

    /// 成功响应（无数据）
    pub fn ok_without_data(message: &str) -> ApiResponse<()> {
        ApiResponse {
            code: 0,
            message: message.to_string(),
            data: None,
        }
    }
}

/// 从错误创建响应
impl ApiResponse<()> {
    pub fn error(code: u16, message: &str) -> Self {
        Self {
            code,
            message: message.to_string(),
            data: None,
        }
    }
}

/// 方便函数：从 StatusCode 创建错误响应
pub fn error_response(status: StatusCode, message: &str) -> ApiResponse<()> {
    ApiResponse::error(status.into(), message)
}
