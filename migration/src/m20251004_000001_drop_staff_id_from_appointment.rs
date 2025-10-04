use sea_orm_migration::prelude::*;
use sea_orm::Statement;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        // Drop the legacy staff_id column if it still exists
        db.execute(Statement::from_string(
            manager.get_database_backend(),
            "ALTER TABLE appointment DROP COLUMN IF EXISTS staff_id;",
        ))
        .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        // Restore the column for rollback scenarios (kept nullable to avoid data issues)
        db.execute(Statement::from_string(
            manager.get_database_backend(),
            "ALTER TABLE appointment ADD COLUMN IF NOT EXISTS staff_id UUID REFERENCES staff(id) ON DELETE CASCADE;",
        ))
        .await?;
        Ok(())
    }
}