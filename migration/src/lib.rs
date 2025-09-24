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
mod m20250623_100000_create_department_table;
mod m20250623_120000_alter_department_name_to_enum;
mod m20250623_130000_add_unique_indexes_patient_staff_department;
mod m20250722_000000_update_appointment_ids_to_uuid;
mod m20250916_000001_add_person_search_indexes;
mod m20250924_000001_create_user_profile;

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
            Box::new(m20250623_100000_create_department_table::Migration),
            Box::new(m20250623_120000_alter_department_name_to_enum::Migration),
            Box::new(m20250623_130000_add_unique_indexes_patient_staff_department::Migration),
            Box::new(m20250722_000000_update_appointment_ids_to_uuid::Migration),
            Box::new(m20250916_000001_add_person_search_indexes::Migration),
            Box::new(m20250924_000001_create_user_profile::Migration),
        ]
    }
}
