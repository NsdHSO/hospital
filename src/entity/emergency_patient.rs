//! SeaORM Entity for emergency_patient join table

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "emergency_patient")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub emergency_id: Uuid,
    #[sea_orm(primary_key, auto_increment = false)]
    pub patient_id: Uuid,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(belongs_to = "super::emergency::Entity", from = "Column::EmergencyId", to = "super::emergency::Column::Id")]
    Emergency,
    #[sea_orm(belongs_to = "super::patient::Entity", from = "Column::PatientId", to = "super::patient::Column::Id")]
    Patient,
}

impl Related<super::emergency::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Emergency.def()
    }
}

impl Related<super::patient::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Patient.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
