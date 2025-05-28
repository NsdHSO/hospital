use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create gender_enum type if it does not exist
        manager
            .get_connection()
            .execute_unprepared(
                "DO $$ BEGIN IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'gender_enum') THEN CREATE TYPE gender_enum AS ENUM ('MALE', 'FEMALE'); END IF; END $$;"
            )
            .await?;
        // Create blood_type_enum type if it does not exist
        manager
            .get_connection()
            .execute_unprepared(
                "DO $$ BEGIN IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'blood_type_enum') THEN CREATE TYPE blood_type_enum AS ENUM ('A_POSITIVE', 'A_NEGATIVE', 'B_POSITIVE', 'B_NEGATIVE', 'AB_POSITIVE', 'AB_NEGATIVE', 'O_POSITIVE', 'O_NEGATIVE'); END IF; END $$;"
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(Patient::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Patient::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Patient::CreatedAt).timestamp().not_null())
                    .col(ColumnDef::new(Patient::UpdatedAt).timestamp().not_null())
                    .col(ColumnDef::new(Patient::HospitalId).integer().not_null())
                    .col(ColumnDef::new(Patient::FirstName).string().not_null())
                    .col(ColumnDef::new(Patient::LastName).string().not_null())
                    .col(ColumnDef::new(Patient::DateOfBirth).date().not_null())
                    .col(ColumnDef::new(Patient::Gender)
                        .custom("gender_enum")
                        .not_null())
                    .col(ColumnDef::new(Patient::Phone).string().not_null())
                    .col(ColumnDef::new(Patient::Email).string().null())
                    .col(ColumnDef::new(Patient::Address).string().not_null())
                    .col(ColumnDef::new(Patient::EmergencyContact).string().null())
                    .col(ColumnDef::new(Patient::BloodType)
                        .custom("blood_type_enum")
                        .null())
                    .col(ColumnDef::new(Patient::Allergies).json().null())
                    .col(ColumnDef::new(Patient::MedicalHistory).text().null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Patient::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum Patient {
    Table,
    Id,
    CreatedAt,
    UpdatedAt,
    HospitalId,
    FirstName,
    LastName,
    DateOfBirth,
    Gender,
    Phone,
    Email,
    Address,
    EmergencyContact,
    BloodType,
    Allergies,
    MedicalHistory,
}
