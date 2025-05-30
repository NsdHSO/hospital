//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.11

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "treatment")]
pub struct Model {
    pub created_at: DateTime,
    pub updated_at: DateTime,
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub admission_id: i32,
    pub doctor_id: i32,
    pub hospital_id: i32,
    #[sea_orm(column_type = "Text")]
    pub description: String,
    pub treatment_date: DateTime,
    #[sea_orm(column_type = "Decimal(Some((10, 2)))")]
    pub cost: Decimal,
    #[sea_orm(column_type = "Text", nullable)]
    pub notes: Option<String>,
    pub treatment_ic: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
