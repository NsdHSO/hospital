//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.11

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "hospital")]
pub struct Model {
    pub created_at: DateTime,
    pub updated_at: DateTime,
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    #[sea_orm(unique)]
    pub name: String,
    pub address: String,
    pub phone: Option<String>,
    pub website: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub description: Option<String>,
    pub capacity: Option<i32>,
    pub established: Option<i32>,
    pub ceo: Option<String>,
    pub trauma_level: Option<String>,
    pub revenue: Option<i32>,
    pub non_profit: Option<bool>,
    pub license_number: Option<String>,
    pub accreditation: Option<String>,
    pub patient_satisfaction_rating: Option<i32>,
    pub average_stay_length: Option<i32>,
    pub annual_budget: Option<i32>,
    pub owner: Option<String>,
    pub latitude: Option<Decimal>,
    pub longitude: Option<Decimal>,
    pub hospital_ic: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct HospitalRequestBody {
    pub name: Option<String>,
    pub address: Option<String>,
    pub website: Option<String>,
    pub description: Option<String>,
    pub capacity: Option<i32>,
    pub ceo: Option<String>,
    pub phone: Option<String>,
    #[allow(non_snake_case)]
    pub trauma_level: Option<String>,
    #[allow(non_snake_case)]
    pub non_profit: Option<bool>,
    #[allow(non_snake_case)]
    pub license_number: Option<String>,
    pub accreditation: Option<String>,
    pub owner: Option<String>,
    pub latitude: Option<Decimal>,
    pub longitude: Option<Decimal>,
}
