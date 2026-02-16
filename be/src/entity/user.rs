use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt;

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

// 自定义 Debug 实现，隐藏 password 字段；不用实现 #[derive(Debug)]
impl fmt::Debug for Model {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Model")
            .field("id", &self.id)
            .field("username", &self.username)
            .field("password", &"[REDACTED]")
            .field("age", &self.age)
            .field("gender", &self.gender)
            .field("email", &self.email)
            .field("created_at", &self.created_at)
            .field("updated_at", &self.updated_at)
            .field("is_deleted", &self.is_deleted)
            .finish()
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
