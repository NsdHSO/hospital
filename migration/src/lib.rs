pub use sea_orm_migration::prelude::*;

mod first_24_05_2025;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(first_24_05_2025::Migration),
        ]
    }
}