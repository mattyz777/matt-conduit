//! 用户数据访问层 (DAO)

use crate::entity::user::{self, Entity as UserEntity, Gender};
use crate::state::AppState;
use crate::error::AppError;
use sea_orm::{Set, EntityTrait, ColumnTrait, IntoActiveModel, QueryFilter, ActiveModelTrait};

pub struct UserDao;

impl UserDao {
    /// 插入新用户
    pub async fn insert(
        state: &AppState,
        username: String,
        hashed_password: String,
        age: Option<i32>,
        gender: Gender,
        email: Option<String>,
    ) -> Result<user::Model, AppError> {
        let user = user::ActiveModel {
            username: Set(username),
            password: Set(hashed_password),
            age: Set(age),
            gender: Set(gender),
            email: Set(email),
            is_deleted: Set(false),
            ..Default::default()
        }
        .insert(&state.db)
        .await?;

        Ok(user)
    }

    /// 更新用户
    pub async fn update(
        state: &AppState,
        user: user::Model,
        username: Option<String>,
        hashed_password: Option<String>,
        age: Option<i32>,
        gender: Option<Gender>,
        email: Option<String>,
    ) -> Result<user::Model, AppError> {
        let mut user_active = user.into_active_model();

        if let Some(new_username) = username {
            user_active.username = Set(new_username);
        }

        if let Some(new_password) = hashed_password {
            user_active.password = Set(new_password);
        }

        if let Some(new_age) = age {
            user_active.age = Set(Some(new_age));
        }

        if let Some(new_gender) = gender {
            user_active.gender = Set(new_gender);
        }

        if let Some(new_email) = email {
            user_active.email = Set(Some(new_email));
        }

        user_active.updated_at = Set(chrono::Utc::now().naive_utc());

        let updated_user = user_active.update(&state.db).await?;

        Ok(updated_user)
    }


    /// 根据 ID 查询用户
    pub async fn find_by_id(
        state: &AppState,
        id: i32,
    ) -> Result<Option<user::Model>, AppError> {
        let user = UserEntity::find()
            .filter(user::Column::Id.eq(id))
            .filter(user::Column::IsDeleted.eq(false))
            .one(&state.db)
            .await?;

        Ok(user)
    }

    /// 根据用户名查询用户
    pub async fn find_by_username(
        state: &AppState,
        username: &str,
    ) -> Result<Option<user::Model>, AppError> {
        let user = UserEntity::find()
            .filter(user::Column::Username.eq(username))
            .filter(user::Column::IsDeleted.eq(false))
            .one(&state.db)
            .await?;

        Ok(user)
    }

    /// 检查用户名是否已存在（排除指定 ID）
    pub async fn exists_by_username(
        state: &AppState,
        username: &str,
        exclude_id: Option<i32>,
    ) -> Result<bool, AppError> {
        let mut query = UserEntity::find()
            .filter(user::Column::Username.eq(username))
            .filter(user::Column::IsDeleted.eq(false));

        if let Some(id) = exclude_id {
            query = query.filter(user::Column::Id.ne(id));
        }

        let existing = query.one(&state.db).await?;
        Ok(existing.is_some())
    }

    /// 软删除用户（设置 is_deleted = true）
    pub async fn soft_delete(
        state: &AppState,
        user: user::Model,
    ) -> Result<(), AppError> {
        let mut user_active = user.into_active_model();

        user_active.is_deleted = Set(true);
        user_active.updated_at = Set(chrono::Utc::now().naive_utc());

        user_active.update(&state.db).await?;

        Ok(())
    }
}
