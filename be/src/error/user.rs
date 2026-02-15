//! 用户相关错误类型
//!
//! 提供用户模块特定的错误类型定义

use thiserror::Error;

/// 用户相关错误
#[derive(Error, Debug)]
pub enum UserError {
    #[error("User not found")]
    NotFound(String),

    #[error("User already exists")]
    AlreadyExists(String),

    #[error("Invalid password")]
    InvalidPassword(String),

    #[error("Password hash error")]
    PasswordHashError(String),
}
