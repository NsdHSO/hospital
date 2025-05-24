use sea_orm_migration::prelude::*;
use ::sea_orm::Statement;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        // Add default value for createdAt column
        db.execute(Statement::from_string(
            manager.get_database_backend(),
            r#"ALTER TABLE dashboard ALTER COLUMN "createdAt" SET DEFAULT CURRENT_TIMESTAMP;"#,
        ))
        .await?;

        // Add default value for updatedAt column
        db.execute(Statement::from_string(
            manager.get_database_backend(),
            r#"ALTER TABLE dashboard ALTER COLUMN "updatedAt" SET DEFAULT CURRENT_TIMESTAMP;"#,
        ))
        .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        // Remove default value for createdAt column
        db.execute(Statement::from_string(
            manager.get_database_backend(),
            r#"ALTER TABLE dashboard ALTER COLUMN "createdAt" DROP DEFAULT;"#,
        ))
        .await?;

        // Remove default value for updatedAt column
        db.execute(Statement::from_string(
            manager.get_database_backend(),
            r#"ALTER TABLE dashboard ALTER COLUMN "updatedAt" DROP DEFAULT;"#,
        ))
        .await?;

        Ok(())
    }
}

