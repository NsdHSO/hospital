use sea_orm_migration::prelude::*;
use ::sea_orm::Statement;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        // Add missing columns (only if they do not already exist)
        db.execute(Statement::from_string(
            manager.get_database_backend(),
            r#"DO $$ BEGIN IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name='card' AND column_name='icon') THEN ALTER TABLE card ADD COLUMN icon VARCHAR; END IF; END $$;"#,
        ))
        .await?;

        db.execute(Statement::from_string(
            manager.get_database_backend(),
            r#"DO $$ BEGIN IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name='card' AND column_name='position') THEN ALTER TABLE card ADD COLUMN position INTEGER; END IF; END $$;"#,
        ))
        .await?;

        db.execute(Statement::from_string(
            manager.get_database_backend(),
            r#"DO $$ BEGIN IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name='card' AND column_name='dataConfig') THEN ALTER TABLE card ADD COLUMN "dataConfig" JSONB; END IF; END $$;"#,
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

