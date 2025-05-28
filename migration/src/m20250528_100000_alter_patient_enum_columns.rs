use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Ensure gender_enum exists
        manager
            .get_connection()
            .execute_unprepared(
                "DO $$ BEGIN IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'gender_enum') THEN CREATE TYPE gender_enum AS ENUM ('MALE', 'FEMALE'); END IF; END $$;"
            )
            .await?;
        // Ensure blood_type_enum exists
        manager
            .get_connection()
            .execute_unprepared(
                "DO $$ BEGIN IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'blood_type_enum') THEN CREATE TYPE blood_type_enum AS ENUM ('A_POSITIVE', 'A_NEGATIVE', 'B_POSITIVE', 'B_NEGATIVE', 'AB_POSITIVE', 'AB_NEGATIVE', 'O_POSITIVE', 'O_NEGATIVE'); END IF; END $$;"
            )
            .await?;
        // Alter gender column to use gender_enum
        manager
            .get_connection()
            .execute_unprepared(
                "ALTER TABLE patient ALTER COLUMN \"gender\" TYPE gender_enum USING \"gender\"::gender_enum;"
            )
            .await?;
        // Alter bloodType column to use blood_type_enum
        manager
            .get_connection()
            .execute_unprepared(
                "ALTER TABLE patient ALTER COLUMN \"bloodType\" TYPE blood_type_enum USING \"bloodType\"::blood_type_enum;"
            )
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Optionally revert columns to text (if needed)
        manager
            .get_connection()
            .execute_unprepared(
                "ALTER TABLE patient ALTER COLUMN \"gender\" TYPE text USING \"gender\"::text;"
            )
            .await?;
        manager
            .get_connection()
            .execute_unprepared(
                "ALTER TABLE patient ALTER COLUMN \"bloodType\" TYPE text USING \"bloodType\"::text;"
            )
            .await?;
        Ok(())
    }
}
