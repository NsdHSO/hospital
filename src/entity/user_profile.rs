use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "user_profile")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub user_sub: String,
    pub schema_version: i32,
    #[sea_orm(column_type = "JsonBinary")]
    pub attributes_json: Json,
    pub updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
