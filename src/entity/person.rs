//! SeaORM Entity for person (shared fields)

use sea_orm::entity::prelude::*;
use crate::entity::sea_orm_active_enums::{GenderEnum, BloodTypeEnum};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "person")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub date_of_birth: Option<Date>,
    pub gender: Option<GenderEnum>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub address: Option<String>,
    pub nationality: Option<String>,
    pub marital_status: Option<String>,
    pub photo_url: Option<String>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct PersonRequestBody{
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub date_of_birth: Option<Date>,
    pub gender: Option<GenderEnum>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub address: Option<String>,
    pub nationality: Option<String>,
    pub marital_status: Option<String>,
    pub photo_url: Option<String>,
}
