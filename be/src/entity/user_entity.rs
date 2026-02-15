use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

// 使用统一的敏感字段 Debug 实现
use crate::impl_redacted_debug;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, EnumIter, DeriveActiveEnum)]
// #[sea_orm(rs_type = "i16", db_type = "TinyInteger")] // mysql/postgres
#[sea_orm(rs_type = "i16", db_type = "SmallInteger")] // supabase
pub enum Gender {
    #[sea_orm(num_value = 1)]
    Male,
    #[sea_orm(num_value = 2)]
    Female,
}

#[derive(Clone, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "t_users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub username: String,
    #[serde(skip_serializing)]
    pub password: String,
    pub age: Option<i32>,
    pub gender: Gender,
    pub email: Option<String>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub is_deleted: bool,
}

// 编译时自动展开成代码
impl_redacted_debug!(Model, password, [id, username, age, gender, email, created_at, updated_at, is_deleted]);

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
