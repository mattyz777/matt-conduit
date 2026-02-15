//! 用户服务层
//!
//! 负责用户相关的业务逻辑

use crate::entity::user_entity::{self, Gender};
use crate::state::AppState;
use crate::error::AppError;
use crate::utils::crypto;
use crate::dao::UserDao;

/// 用户服务
pub struct UserService;

impl UserService {
    /// 创建用户
    pub async fn create(
        state: &AppState,
        username: String,
        password: String,
        age: Option<i32>,
        gender: Gender,
        email: Option<String>,
    ) -> Result<user_entity::Model, AppError> {
        // 检查用户名是否已存在
        let exists = UserDao::exists_by_username(state, &username, None).await?;
        if exists {
            return Err(AppError::AlreadyExists(format!("用户名已存在: {}", username)));
        }

        // 哈希密码
        let hashed_password = crypto::hash_password(&password)
            .map_err(|e| AppError::InternalServer(format!("密码哈希失败: {}", e)))?;

        // 插入用户
        let user = UserDao::insert(
            state,
            username,
            hashed_password,
            age,
            gender,
            email,
        )
        .await?;

        Ok(user)
    }

    /// 根据 ID 查询用户
    pub async fn find_by_id(state: &AppState, id: i32) -> Result<Option<user_entity::Model>, AppError> {
        UserDao::find_by_id(state, id).await
    }

    /// 根据用户名查询用户
    pub async fn find_by_username(
        state: &AppState,
        username: &str,
    ) -> Result<user_entity::Model, AppError> {
        let user = UserDao::find_by_username(state, username).await?
            .ok_or(AppError::NotFound(format!("用户不存在: {}", username)))?;

        Ok(user)
    }

    /// 更新用户
    pub async fn update(
        state: &AppState,
        id: i32,
        username: Option<String>,
        password: Option<String>,
        age: Option<Option<i32>>,
        gender: Option<Gender>,
        email: Option<Option<String>>,
    ) -> Result<user_entity::Model, AppError> {
        // 查找用户
        let user = Self::find_by_id(state, id).await?
            .ok_or(AppError::NotFound(format!("用户不存在: {}", id)))?;

        // 检查新用户名是否已被其他用户占用
        if let Some(ref new_username) = username {
            let exists = UserDao::exists_by_username(state, new_username, Some(id)).await?;
            if exists {
                return Err(AppError::AlreadyExists(format!("用户名已存在: {}", new_username)));
            }
        }

        // 哈希新密码（如果提供）
        let hashed_password = match password {
            Some(pwd) => Some(crypto::hash_password(&pwd)
                .map_err(|e| AppError::InternalServer(format!("密码哈希失败: {}", e)))?),
            None => None,
        };

        // 更新用户
        UserDao::update(
            state,
            user,
            username,
            hashed_password,
            age,
            gender,
            email,
        )
        .await
    }

    /// 删除用户（软删除）
    pub async fn delete(state: &AppState, id: i32) -> Result<(), AppError> {
        let user = Self::find_by_id(state, id).await?
            .ok_or(AppError::NotFound(format!("用户不存在: {}", id)))?;

        UserDao::soft_delete(state, user).await
    }

    /// 验证用户登录
    pub async fn verify_login(
        state: &AppState,
        username: &str,
        password: &str,
    ) -> Result<user_entity::Model, AppError> {
        // 查找用户
        let user = Self::find_by_username(state, username).await?;

        // 验证密码
        let is_valid = crypto::verify_password(password, &user.password)
            .map_err(|e| AppError::InternalServer(format!("密码验证失败: {}", e)))?;

        if !is_valid {
            return Err(AppError::Unauthorized("密码错误".to_string()));
        }

        Ok(user)
    }
}
