use crate::entity::Gender;
use serde::{Deserialize, Serialize};

/// 用户创建请求
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CreateUserRequest {
    pub username: String,
    pub password: String,
    pub age: Option<i32>,
    pub gender: Gender,
    pub email: Option<String>,
}

/// 用户更新请求
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UpdateUserRequest {
    pub username: Option<String>,
    pub password: Option<String>,
    pub age: Option<Option<i32>>,
    pub gender: Option<Gender>,
    pub email: Option<Option<String>>,
}

/// 用户响应（不含密码）
#[derive(Debug, Clone, Serialize)]
pub struct UserResponse {
    pub id: i32,
    pub username: String,
    pub age: Option<i32>,
    pub gender: Gender,
    pub email: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

/// 用户登录请求
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

/// 用户列表响应
#[derive(Debug, Clone, Serialize)]
pub struct UserListResponse {
    pub users: Vec<UserResponse>,
    pub total: usize,
}
