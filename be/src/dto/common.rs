//! 通用 DTO 定义
//! Option 序列化为 JSON 时 None 变成 null
use serde::Serialize;

/// 统一的 API 响应结构
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
#[derive(Debug, Clone, Serialize)]
pub struct ApiResponse<T> {
    pub code: u16,
    pub message: String,
    pub data: Option<T>,
}

impl<T> ApiResponse<T> {
    pub fn ok(data: T) -> Self {
        Self {
            code: 0,
            message: "success".to_string(),
            data: Some(data),
        }
    }

    pub fn ok_with_message(data: T, message: &str) -> Self {
        Self {
            code: 0,
            message: message.to_string(),
            data: Some(data),
        }
    }
}

impl ApiResponse<()> {
    pub fn error(code: u16, message: &str) -> Self {
        Self {
            code,
            message: message.to_string(),
            data: None,
        }
    }

    pub fn ok_without_data(message: &str) -> Self {
        Self {
            code: 0,
            message: message.to_string(),
            data: None,
        }
    }
}

