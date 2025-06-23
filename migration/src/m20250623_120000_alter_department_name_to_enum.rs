use sea_orm_migration::prelude::*;
use sea_orm::Statement;
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Ensure all department.name values are valid ENUM values before altering type
        // (You should manually check and fix any invalid values before running this migration)

        // Alter the column type from VARCHAR to ENUM
        manager
            .get_connection()
            .execute_unprepared(
                "ALTER TABLE department ALTER COLUMN name TYPE department_name_enum USING name::department_name_enum;",
            )
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Revert the column type back to VARCHAR
        manager
            .get_connection()
            .execute_unprepared("ALTER TABLE department ALTER COLUMN name TYPE VARCHAR;")
            .await?;
        Ok(())
    }
}
