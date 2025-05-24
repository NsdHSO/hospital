pub use sea_orm_migration::prelude::*;

mod m20250524_000001_initial_setup;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250524_000001_initial_setup::Migration),
        ]
    }
}