use sea_orm_migration::prelude::*;
use ::sea_orm::Statement;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        // Add missing columns
        db.execute(Statement::from_string(
            manager.get_database_backend(),
            r#"ALTER TABLE card ADD COLUMN icon VARCHAR;"#,
        ))
        .await?;

        db.execute(Statement::from_string(
            manager.get_database_backend(),
            r#"ALTER TABLE card ADD COLUMN position INTEGER;"#,
        ))
        .await?;

        db.execute(Statement::from_string(
            manager.get_database_backend(),
            r#"ALTER TABLE card ADD COLUMN "dataConfig" JSONB;"#,
        ))
        .await?;

        // Make columns nullable to match the entity model
        db.execute(Statement::from_string(
            manager.get_database_backend(),
            r#"ALTER TABLE card ALTER COLUMN "dashboardId" DROP NOT NULL;"#,
        ))
        .await?;

        db.execute(Statement::from_string(
            manager.get_database_backend(),
            r#"ALTER TABLE card ALTER COLUMN "cardType" DROP NOT NULL;"#,
        ))
        .await?;

        db.execute(Statement::from_string(
            manager.get_database_backend(),
            r#"ALTER TABLE card ALTER COLUMN size DROP NOT NULL;"#,
        ))
        .await?;

        // Add default values for timestamp columns
        db.execute(Statement::from_string(
            manager.get_database_backend(),
            r#"ALTER TABLE card ALTER COLUMN "createdAt" SET DEFAULT CURRENT_TIMESTAMP;"#,
        ))
        .await?;

        db.execute(Statement::from_string(
            manager.get_database_backend(),
            r#"ALTER TABLE card ALTER COLUMN "updatedAt" SET DEFAULT CURRENT_TIMESTAMP;"#,
        ))
        .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        // Remove default values for timestamp columns
        db.execute(Statement::from_string(
            manager.get_database_backend(),
            r#"ALTER TABLE card ALTER COLUMN "createdAt" DROP DEFAULT;"#,
        ))
        .await?;

        db.execute(Statement::from_string(
            manager.get_database_backend(),
            r#"ALTER TABLE card ALTER COLUMN "updatedAt" DROP DEFAULT;"#,
        ))
        .await?;

        // Make columns NOT NULL again
        db.execute(Statement::from_string(
            manager.get_database_backend(),
            r#"ALTER TABLE card ALTER COLUMN "dashboardId" SET NOT NULL;"#,
        ))
        .await?;

        db.execute(Statement::from_string(
            manager.get_database_backend(),
            r#"ALTER TABLE card ALTER COLUMN "cardType" SET NOT NULL;"#,
        ))
        .await?;

        db.execute(Statement::from_string(
            manager.get_database_backend(),
            r#"ALTER TABLE card ALTER COLUMN size SET NOT NULL;"#,
        ))
        .await?;

        // Drop added columns
        db.execute(Statement::from_string(
            manager.get_database_backend(),
            r#"ALTER TABLE card DROP COLUMN "dataConfig";"#,
        ))
        .await?;

        db.execute(Statement::from_string(
            manager.get_database_backend(),
            r#"ALTER TABLE card DROP COLUMN position;"#,
        ))
        .await?;

        db.execute(Statement::from_string(
            manager.get_database_backend(),
            r#"ALTER TABLE card DROP COLUMN icon;"#,
        ))
        .await?;

        Ok(())
    }
}

