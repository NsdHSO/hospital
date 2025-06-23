pub use sea_orm_migration::prelude::*;

mod m20250524_000001_initial_setup;
mod m20250524_122219_update_dashboard;
mod m20250524_122220_update_dashboard;
mod m20250524_123225_make_userid_nullable;
mod m20250524_123751_add_timestamp_defaults;
mod m20250524_124017_update_card;
mod m20250528_210000_add_ic_columns_if_missing;
mod m20250529_000001_create_emergency_patient_table;
mod m20250621_000001_add_person_table_and_refactor_patient_staff;
mod m20250621_100000_add_specialization_to_staff;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250524_000001_initial_setup::Migration),
            Box::new(m20250524_122219_update_dashboard::Migration),
            Box::new(m20250524_122220_update_dashboard::Migration),
            Box::new(m20250524_123225_make_userid_nullable::Migration),
            Box::new(m20250524_123751_add_timestamp_defaults::Migration),
            Box::new(m20250524_124017_update_card::Migration),
            Box::new(m20250528_210000_add_ic_columns_if_missing::Migration),
            Box::new(m20250529_000001_create_emergency_patient_table::Migration),
            Box::new(m20250621_000001_add_person_table_and_refactor_patient_staff::Migration),
            Box::new(m20250621_100000_add_specialization_to_staff::Migration),
        ]
    }
}
