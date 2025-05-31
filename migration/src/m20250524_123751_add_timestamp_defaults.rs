use sea_orm_migration::prelude::*;
use ::sea_orm::Statement;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        // Add default value for created_at column
        db.execute(Statement::from_string(
            manager.get_database_backend(),
            r#"ALTER TABLE dashboard ALTER COLUMN created_at SET DEFAULT CURRENT_TIMESTAMP;"#,
        ))
        .await?;

        // Add default value for updated_at column
        db.execute(Statement::from_string(
            manager.get_database_backend(),
            r#"ALTER TABLE dashboard ALTER COLUMN updated_at SET DEFAULT CURRENT_TIMESTAMP;"#,
        ))
        .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        // Remove default value for created_at column
        db.execute(Statement::from_string(
            manager.get_database_backend(),
            r#"ALTER TABLE dashboard ALTER COLUMN created_at DROP DEFAULT;"#,
        ))
        .await?;

        // Remove default value for updated_at column
        db.execute(Statement::from_string(
            manager.get_database_backend(),
            r#"ALTER TABLE dashboard ALTER COLUMN updated_at DROP DEFAULT;"#,
        ))
        .await?;

        Ok(())
    }
}

