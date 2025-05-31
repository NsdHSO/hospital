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
            r#"DO $$ BEGIN IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name='card' AND column_name='data_config') THEN ALTER TABLE card ADD COLUMN data_config JSONB; END IF; END $$;"#,
        ))
        .await?;

        db.execute(Statement::from_string(
            manager.get_database_backend(),
            r#"ALTER TABLE card ALTER COLUMN dashboard_id DROP NOT NULL;"#,
        ))
        .await?;

        db.execute(Statement::from_string(
            manager.get_database_backend(),
            r#"ALTER TABLE card ALTER COLUMN card_type DROP NOT NULL;"#,
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
            r#"ALTER TABLE card ALTER COLUMN created_at SET DEFAULT CURRENT_TIMESTAMP;"#,
        ))
        .await?;

        db.execute(Statement::from_string(
            manager.get_database_backend(),
            r#"ALTER TABLE card ALTER COLUMN updated_at SET DEFAULT CURRENT_TIMESTAMP;"#,
        ))
        .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        // Remove default values for timestamp columns
        db.execute(Statement::from_string(
            manager.get_database_backend(),
            r#"ALTER TABLE card ALTER COLUMN created_at DROP DEFAULT;"#,
        ))
        .await?;

        db.execute(Statement::from_string(
            manager.get_database_backend(),
            r#"ALTER TABLE card ALTER COLUMN updated_at DROP DEFAULT;"#,
        ))
        .await?;

        // Make columns NOT NULL again
        db.execute(Statement::from_string(
            manager.get_database_backend(),
            r#"ALTER TABLE card ALTER COLUMN dashboard_id SET NOT NULL;"#,
        ))
        .await?;

        db.execute(Statement::from_string(
            manager.get_database_backend(),
            r#"ALTER TABLE card ALTER COLUMN card_type SET NOT NULL;"#,
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
            r#"ALTER TABLE card DROP COLUMN data_config;"#,
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

