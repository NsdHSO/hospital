use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create person table
        manager.create_table(
            Table::create()
                .table(Person::Table)
                .if_not_exists()
                .col(ColumnDef::new(Person::Id).uuid().not_null().primary_key())
                .col(ColumnDef::new(Person::FirstName).string().not_null())
                .col(ColumnDef::new(Person::LastName).string().not_null())
                .col(ColumnDef::new(Person::DateOfBirth).date())
                .col(ColumnDef::new(Person::Gender).string())
                .col(ColumnDef::new(Person::Phone).string())
                .col(ColumnDef::new(Person::Email).string())
                .col(ColumnDef::new(Person::Address).string())
                .col(ColumnDef::new(Person::Nationality).string())
                .col(ColumnDef::new(Person::MaritalStatus).string())
                .col(ColumnDef::new(Person::PhotoUrl).string())
                .col(ColumnDef::new(Person::CreatedAt).date_time().not_null())
                .col(ColumnDef::new(Person::UpdatedAt).date_time().not_null())
                .to_owned()
        ).await?;

        // Backfill person table from patient
        manager.get_connection().execute_unprepared(
            r#"
            INSERT INTO person (id, first_name, last_name, date_of_birth, gender, phone, email, address, created_at, updated_at)
            SELECT id, first_name, last_name, date_of_birth, gender, phone, email, NULL as address, created_at, updated_at FROM patient
            ON CONFLICT (id) DO NOTHING;
            "#
        ).await?;

        // Backfill person table from staff (if not already present)
        manager.get_connection().execute_unprepared(
            r#"
            INSERT INTO person (id, first_name, last_name, phone, email, address, created_at, updated_at)
            SELECT id, name, '' as last_name, phone, email, NULL as address, created_at, updated_at FROM staff
            ON CONFLICT (id) DO NOTHING;
            "#
        ).await?;

        // Alter patient and staff tables to use person id as PK and FK
        manager.alter_table(
            Table::alter()
                .table(Patient::Table)
                .drop_column("first_name")
                .drop_column("last_name")
                .drop_column("date_of_birth")
                .drop_column("gender")
                .drop_column("phone")
                .drop_column("email")
                .drop_column("address")
                .add_foreign_key(
                    TableForeignKey::new()
                        .from_tbl(Patient::Table)
                        .from_col(Patient::Id)
                        .to_tbl(Person::Table)
                        .to_col(Person::Id)
                        .on_delete(ForeignKeyAction::Cascade)
                )
                .to_owned()
        ).await?;

        // Drop specialization column from staff if it exists (robust to missing column)
        manager.get_connection().execute_unprepared(
            r#"ALTER TABLE staff DROP COLUMN IF EXISTS specialization;"#
        ).await?;

        // Drop address column from staff if it exists (robust to missing column)
        manager.get_connection().execute_unprepared(
            r#"ALTER TABLE staff DROP COLUMN IF EXISTS address;"#
        ).await?;

        // Now alter staff table as before, but without .drop_column("address")
        manager.alter_table(
            Table::alter()
                .table(Staff::Table)
                .drop_column("name")
                .drop_column("phone")
                .drop_column("email")
                .add_foreign_key(
                    TableForeignKey::new()
                        .from_tbl(Staff::Table)
                        .from_col(Staff::Id)
                        .to_tbl(Person::Table)
                        .to_col(Person::Id)
                        .on_delete(ForeignKeyAction::Cascade)
                )
                .to_owned()
        ).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Drop foreign keys and columns, then drop person table
        manager.alter_table(
            Table::alter()
                .table(Patient::Table)
                .drop_foreign_key(Alias::new("fk-patient-person-id"))
                .add_column(ColumnDef::new(Patient::FirstName).string().not_null())
                .add_column(ColumnDef::new(Patient::LastName).string().not_null())
                .add_column(ColumnDef::new(Patient::DateOfBirth).date())
                .add_column(ColumnDef::new(Patient::Gender).string())
                .add_column(ColumnDef::new(Patient::Phone).string())
                .add_column(ColumnDef::new(Patient::Email).string())
                .add_column(ColumnDef::new(Patient::Address).string())
                .to_owned()
        ).await?;

        manager.alter_table(
            Table::alter()
                .table(Staff::Table)
                .drop_foreign_key(Alias::new("fk-staff-person-id"))
                .add_column(ColumnDef::new(Staff::Name).string().not_null())
                .add_column(ColumnDef::new(Staff::Phone).string())
                .add_column(ColumnDef::new(Staff::Email).string())
                .add_column(ColumnDef::new(Staff::Address).string())
                .to_owned()
        ).await?;

        manager.drop_table(Table::drop().table(Person::Table).to_owned()).await?;
        Ok(())
    }
}

#[derive(Iden)]
enum Person {
    Table,
    Id,
    FirstName,
    LastName,
    DateOfBirth,
    Gender,
    Phone,
    Email,
    Address,
    Nationality,
    MaritalStatus,
    PhotoUrl,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
enum Patient {
    Table,
    Id,
    FirstName,
    LastName,
    DateOfBirth,
    Gender,
    Phone,
    Email,
    Address,
}

#[derive(Iden)]
enum Staff {
    Table,
    Id,
    Name,
    Phone,
    Email,
    Address,
}
