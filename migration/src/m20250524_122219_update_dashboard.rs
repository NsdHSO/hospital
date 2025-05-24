use sea_orm_migration::prelude::*;
use ::sea_orm::Statement;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        // Step 1: Rename title column to name
        db.execute(Statement::from_string(
            manager.get_database_backend(),
            r#"ALTER TABLE dashboard RENAME COLUMN title TO name;"#,
        ))
        .await?;

        // Step 2: Add is_active column with default value of true
        db.execute(Statement::from_string(
            manager.get_database_backend(),
            r#"ALTER TABLE dashboard ADD COLUMN "isActive" BOOLEAN NOT NULL DEFAULT true;"#,
        ))
        .await?;

        // Step 3: Make userId nullable (since owner_id is optional in the entity)
        db.execute(Statement::from_string(
            manager.get_database_backend(),
            r#"ALTER TABLE dashboard ALTER COLUMN "userId" DROP NOT NULL;"#,
        ))
        .await?;

        // Step 4: Add owner_id column (optional integer)
        db.execute(Statement::from_string(
            manager.get_database_backend(),
            r#"ALTER TABLE dashboard ADD COLUMN "ownerId" INTEGER;"#,
        ))
        .await?;

        // Step 5: Add layout_config column (optional string)
        db.execute(Statement::from_string(
            manager.get_database_backend(),
            r#"ALTER TABLE dashboard ADD COLUMN "layoutConfig" TEXT;"#,
        ))
        .await?;

        // The existing columns id, description, createdAt, and updatedAt remain unchanged

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        // Step 1: Remove layout_config column
        db.execute(Statement::from_string(
            manager.get_database_backend(),
            r#"ALTER TABLE dashboard DROP COLUMN "layoutConfig";"#,
        ))
        .await?;

        // Step 2: Remove owner_id column
        db.execute(Statement::from_string(
            manager.get_database_backend(),
            r#"ALTER TABLE dashboard DROP COLUMN "ownerId";"#,
        ))
        .await?;

        // Step 3: Make userId NOT NULL again
        db.execute(Statement::from_string(
            manager.get_database_backend(),
            r#"ALTER TABLE dashboard ALTER COLUMN "userId" SET NOT NULL;"#,
        ))
        .await?;

        // Step 4: Remove is_active column
        db.execute(Statement::from_string(
            manager.get_database_backend(),
            r#"ALTER TABLE dashboard DROP COLUMN "isActive";"#,
        ))
        .await?;

        // Step 5: Rename name column back to title
        db.execute(Statement::from_string(
            manager.get_database_backend(),
            r#"ALTER TABLE dashboard RENAME COLUMN name TO title;"#,
        ))
        .await?;

        Ok(())
    }
}

