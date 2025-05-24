use sea_orm_migration::prelude::*;
use ::sea_orm::Statement;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        // Make userId column nullable
        db.execute(Statement::from_string(
            manager.get_database_backend(),
            r#"ALTER TABLE dashboard ALTER COLUMN "userId" DROP NOT NULL;"#,
        ))
        .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        // Make userId column NOT NULL again
        db.execute(Statement::from_string(
            manager.get_database_backend(),
            r#"ALTER TABLE dashboard ALTER COLUMN "userId" SET NOT NULL;"#,
        ))
        .await?;

        Ok(())
    }
}

