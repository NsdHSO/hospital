use sea_orm_migration::prelude::*;
use ::sea_orm::Statement;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        // Step 1: Check if title column exists and rename it to name, otherwise add name column
        db.execute(Statement::from_string(
            manager.get_database_backend(),
            r#"DO $$ 
            BEGIN 
                IF EXISTS (
                    SELECT 1 
                    FROM information_schema.columns 
                    WHERE table_name = 'dashboard' AND column_name = 'title'
                ) THEN
                    ALTER TABLE dashboard RENAME COLUMN title TO name;
                ELSIF NOT EXISTS (
                    SELECT 1 
                    FROM information_schema.columns 
                    WHERE table_name = 'dashboard' AND column_name = 'name'
                ) THEN
                    ALTER TABLE dashboard ADD COLUMN name VARCHAR NOT NULL DEFAULT '';
                END IF;
            END $$;"#,
        ))
        .await?;

        // Step 2: Add is_active column with default value of true if it doesn't exist
        db.execute(Statement::from_string(
            manager.get_database_backend(),
            r#"DO $$ 
            BEGIN 
                IF NOT EXISTS (
                    SELECT 1 
                    FROM information_schema.columns 
                    WHERE table_name = 'dashboard' AND column_name = 'is_active'
                ) THEN
                    ALTER TABLE dashboard ADD COLUMN is_active BOOLEAN NOT NULL DEFAULT true;
                END IF;
            END $$;"#,
        ))
        .await?;

        // Step 3: Make userId nullable if it's not already nullable
        db.execute(Statement::from_string(
            manager.get_database_backend(),
            r#"DO $$ 
            BEGIN 
                IF EXISTS (
                    SELECT 1 
                    FROM information_schema.columns 
                    WHERE table_name = 'dashboard' 
                    AND column_name = 'user_id' 
                    AND is_nullable = 'NO'
                ) THEN
                    ALTER TABLE dashboard ALTER COLUMN user_id DROP NOT NULL;
                END IF;
            END $$;"#,
        ))
        .await?;

        // Step 4: Add owner_id column (optional integer) if it doesn't exist
        db.execute(Statement::from_string(
            manager.get_database_backend(),
            r#"DO $$ 
            BEGIN 
                IF NOT EXISTS (
                    SELECT 1 
                    FROM information_schema.columns 
                    WHERE table_name = 'dashboard' AND column_name = 'owner_id'
                ) THEN
                    ALTER TABLE dashboard ADD COLUMN owner_id INTEGER;
                END IF;
            END $$;"#,
        ))
        .await?;

        // Step 5: Add layout_config column (optional string) if it doesn't exist
        db.execute(Statement::from_string(
            manager.get_database_backend(),
            r#"DO $$ 
            BEGIN 
                IF NOT EXISTS (
                    SELECT 1 
                    FROM information_schema.columns 
                    WHERE table_name = 'dashboard' AND column_name = 'layout_config'
                ) THEN
                    ALTER TABLE dashboard ADD COLUMN layout_config TEXT;
                END IF;
            END $$;"#,
        ))
        .await?;

        // The existing columns id, description, created_at, and updated_at remain unchanged

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        // Step 1: Remove layout_config column if it exists
        db.execute(Statement::from_string(
            manager.get_database_backend(),
            r#"DO $$ 
            BEGIN 
                IF EXISTS (
                    SELECT 1 
                    FROM information_schema.columns 
                    WHERE table_name = 'dashboard' AND column_name = 'layout_config'
                ) THEN
                    ALTER TABLE dashboard DROP COLUMN layout_config;
                END IF;
            END $$;"#,
        ))
        .await?;

        // Step 2: Remove owner_id column if it exists
        db.execute(Statement::from_string(
            manager.get_database_backend(),
            r#"DO $$ 
            BEGIN 
                IF EXISTS (
                    SELECT 1 
                    FROM information_schema.columns 
                    WHERE table_name = 'dashboard' AND column_name = 'owner_id'
                ) THEN
                    ALTER TABLE dashboard DROP COLUMN owner_id;
                END IF;
            END $$;"#,
        ))
        .await?;

        // Step 3: Make user_id NOT NULL again if it's currently nullable
        db.execute(Statement::from_string(
            manager.get_database_backend(),
            r#"DO $$ 
            BEGIN 
                IF EXISTS (
                    SELECT 1 
                    FROM information_schema.columns 
                    WHERE table_name = 'dashboard' 
                    AND column_name = 'user_id' 
                    AND is_nullable = 'YES'
                ) THEN
                    ALTER TABLE dashboard ALTER COLUMN user_id SET NOT NULL;
                END IF;
            END $$;"#,
        ))
        .await?;

        // Step 4: Remove is_active column if it exists
        db.execute(Statement::from_string(
            manager.get_database_backend(),
            r#"DO $$ 
            BEGIN 
                IF EXISTS (
                    SELECT 1 
                    FROM information_schema.columns 
                    WHERE table_name = 'dashboard' AND column_name = 'is_active'
                ) THEN
                    ALTER TABLE dashboard DROP COLUMN "is_active";
                END IF;
            END $$;"#,
        ))
        .await?;

        // Step 5: Check if name column exists and rename it back to title
        db.execute(Statement::from_string(
            manager.get_database_backend(),
            r#"DO $$ 
            BEGIN 
                IF EXISTS (
                    SELECT 1 
                    FROM information_schema.columns 
                    WHERE table_name = 'dashboard' AND column_name = 'name'
                ) THEN
                    ALTER TABLE dashboard RENAME COLUMN name TO title;
                ELSIF NOT EXISTS (
                    SELECT 1 
                    FROM information_schema.columns 
                    WHERE table_name = 'dashboard' AND column_name = 'title'
                ) THEN
                    ALTER TABLE dashboard ADD COLUMN title VARCHAR NOT NULL DEFAULT '';
                END IF;
            END $$;"#,
        ))
        .await?;

        Ok(())
    }
}

