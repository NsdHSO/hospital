use sea_orm_migration::prelude::*;
use ::sea_orm::Statement;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        // Make userId column nullable if it exists and is NOT NULL
        db.execute(Statement::from_string(
            manager.get_database_backend(),
            r#"DO $$ 
            BEGIN 
                IF EXISTS (
                    SELECT 1 
                    FROM information_schema.columns 
                    WHERE table_name = 'dashboard' 
                    AND column_name = 'userId' 
                    AND is_nullable = 'NO'
                ) THEN
                    ALTER TABLE dashboard ALTER COLUMN "userId" DROP NOT NULL;
                END IF;
            END $$;"#,
        ))
        .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        // Make userId column NOT NULL again if it exists and is nullable
        db.execute(Statement::from_string(
            manager.get_database_backend(),
            r#"DO $$ 
            BEGIN 
                IF EXISTS (
                    SELECT 1 
                    FROM information_schema.columns 
                    WHERE table_name = 'dashboard' 
                    AND column_name = 'userId' 
                    AND is_nullable = 'YES'
                ) THEN
                    ALTER TABLE dashboard ALTER COLUMN "userId" SET NOT NULL;
                END IF;
            END $$;"#,
        ))
        .await?;

        Ok(())
    }
}

