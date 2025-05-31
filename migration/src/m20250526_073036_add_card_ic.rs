use sea_orm_migration::prelude::*;
use ::sea_orm::Statement;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        // First check if the card table exists
        let result = db
            .query_one(Statement::from_string(
                manager.get_database_backend(),
                r#"SELECT EXISTS (
                    SELECT FROM information_schema.tables 
                    WHERE table_schema = 'public' 
                    AND table_name = 'card'
                ) AS exists_flag;"#,
            ))
            .await?;
        
        let table_exists = match result {
            Some(query_result) => query_result.try_get::<bool>("", "exists_flag")?,
            None => false, // Default to false if query returned no results
        };


        if !table_exists {
            // Create the card table if it doesn't exist
            db.execute(Statement::from_string(
                manager.get_database_backend(),
                r#"CREATE TABLE card (
                    created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
                    updated_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
                    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
                    title VARCHAR NOT NULL,
                    content VARCHAR NOT NULL,
                    icon VARCHAR,
                    position INTEGER,
                    data_config JSONB,
                    dashboard_id UUID,
                    card_type card_cardtype_enum,
                    size card_size_enum,
                    card_ic INTEGER UNIQUE
                );"#,
            ))
            .await?;
            
            // Return early since we've already created the table with the cardIc column
            return Ok(());
        }

        // If we're here, the table exists, so we need to check if the cardIc column exists
        let result = db
            .query_one(Statement::from_string(
                manager.get_database_backend(),
                r#"SELECT EXISTS (
                    SELECT FROM information_schema.columns 
                    WHERE table_schema = 'public' 
                    AND table_name = 'card'
                    AND column_name = 'cardIc'
                ) AS exists_flag;"#,
            ))
            .await?;
            
        let column_exists = match result {
            Some(query_result) => query_result.try_get::<bool>("", "exists_flag")?,
            None => false, // Default to false if query returned no results
        };

        if !column_exists {
            // Add cardIc column with unique constraint
            db.execute(Statement::from_string(
                manager.get_database_backend(),
                r#"ALTER TABLE card ADD COLUMN "cardIc" INTEGER;"#,
            ))
            .await?;
            
            // Add unique constraint to cardIc column
            db.execute(Statement::from_string(
                manager.get_database_backend(),
                r#"ALTER TABLE card ADD CONSTRAINT card_cardIc_key UNIQUE ("cardIc");"#,
            ))
            .await?;
        }

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        // Check if the table exists before trying to drop the column
        let result = db
            .query_one(Statement::from_string(
                manager.get_database_backend(),
                r#"SELECT EXISTS (
                    SELECT FROM information_schema.tables 
                    WHERE table_schema = 'public' 
                    AND table_name = 'card'
                ) AS exists_flag;"#,
            ))
            .await?;
            
        let table_exists = match result {
            Some(query_result) => query_result.try_get::<bool>("", "exists_flag")?,
            None => false, // Default to false if query returned no results
        };

        if table_exists {
            // Check if the column exists before trying to drop it
            let result = db
                .query_one(Statement::from_string(
                    manager.get_database_backend(),
                    r#"SELECT EXISTS (
                        SELECT FROM information_schema.columns 
                        WHERE table_schema = 'public' 
                        AND table_name = 'card'
                        AND column_name = 'cardIc'
                    ) AS exists_flag;"#,
                ))
                .await?;
                
            let column_exists = match result {
                Some(query_result) => query_result.try_get::<bool>("", "exists_flag")?,
                None => false, // Default to false if query returned no results
            };

            if column_exists {
                // Drop cardIc column
                db.execute(Statement::from_string(
                    manager.get_database_backend(),
                    r#"ALTER TABLE card DROP COLUMN "cardIc";"#,
                ))
                .await?;
            }
        }

        Ok(())
    }
}

