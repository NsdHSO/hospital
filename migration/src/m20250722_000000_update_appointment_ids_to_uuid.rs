use sea_orm_migration::prelude::*;
use sea_orm::{Iden, sea_query::Expr}; // Import Iden and DeriveIden, and Expr for default timestamp

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(Iden)]
enum Appointment {
    Table,
    Id,
    PatientId,
    DoctorId,
    HospitalId,
    AppointmentIc,
    CreatedAt,
    UpdatedAt,
    AppointmentDate,
    Reason,
    Notes,
    Cost,
    ScheduledBy,
    AppointmentType,
    Status,
}

#[derive(Iden)]
enum Patient {
    Table,
    Id,
}

#[derive(Iden)]
enum Staff {
    Table,
    Id,
}

#[derive(Iden)]
enum Hospital {
    Table,
    Id,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Appointment::Table) 
                    .add_column_if_not_exists(
                        ColumnDef::new(Appointment::CreatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()), 
                    )
                    .add_column_if_not_exists(
                        ColumnDef::new(Appointment::UpdatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .add_column_if_not_exists(
                        ColumnDef::new(Appointment::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .add_column_if_not_exists(
                        ColumnDef::new(Appointment::AppointmentIc)
                            .integer()
                            .unique_key()
                            .not_null(),
                    )
                    .add_column_if_not_exists(
                        ColumnDef::new(Appointment::PatientId).uuid().not_null(),
                    )
                    .add_column_if_not_exists(
                        ColumnDef::new(Appointment::DoctorId).uuid().not_null(),
                    )
                    .add_column_if_not_exists(
                        ColumnDef::new(Appointment::HospitalId).uuid().not_null(),
                    )
                    .add_column_if_not_exists(
                        ColumnDef::new(Appointment::AppointmentDate)
                            .timestamp()
                            .not_null(),
                    )
                    .add_column_if_not_exists(ColumnDef::new(Appointment::Reason).text().null())
                    .add_column_if_not_exists(ColumnDef::new(Appointment::Notes).text().null())
                    .add_column_if_not_exists(ColumnDef::new(Appointment::Cost).decimal().not_null())
                    .add_column_if_not_exists(
                        ColumnDef::new(Appointment::ScheduledBy).string().null(),
                    )
                    .add_column_if_not_exists(
                        ColumnDef::new(Appointment::AppointmentType).string().null(),
                    )
                    .add_column_if_not_exists(
                        ColumnDef::new(Appointment::Status).string().not_null(),
                    ) // You may want to use ENUM if defined
                    .to_owned(),
            )
            .await?;

        // 2. Add foreign key constraints separately
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_appointment_patient")
                    .from(Appointment::Table, Appointment::PatientId)
                    .to(Patient::Table, Patient::Id)
                    .on_delete(ForeignKeyAction::Restrict) // Or SetNull, Cascade, NoAction
                    .on_update(ForeignKeyAction::Restrict)
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_appointment_doctor")
                    .from(Appointment::Table, Appointment::DoctorId)
                    .to(Staff::Table, Staff::Id)
                    .on_delete(ForeignKeyAction::Restrict)
                    .on_update(ForeignKeyAction::Restrict)
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_appointment_hospital")
                    .from(Appointment::Table, Appointment::HospitalId)
                    .to(Hospital::Table, Hospital::Id)
                    .on_delete(ForeignKeyAction::Restrict)
                    .on_update(ForeignKeyAction::Restrict)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_foreign_key(ForeignKey::drop().name("fk_appointment_patient").to_owned())
            .await?;
        manager
            .drop_foreign_key(ForeignKey::drop().name("fk_appointment_doctor").to_owned())
            .await?;
        manager
            .drop_foreign_key(ForeignKey::drop().name("fk_appointment_hospital").to_owned())
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Appointment::Table)
                    .drop_column(Appointment::CreatedAt)
                    .drop_column(Appointment::UpdatedAt)
                    .drop_column(Appointment::Id)
                    .drop_column(Appointment::AppointmentIc)
                    .drop_column(Appointment::PatientId)
                    .drop_column(Appointment::DoctorId)
                    .drop_column(Appointment::HospitalId)
                    .drop_column(Appointment::AppointmentDate)
                    .drop_column(Appointment::Reason)
                    .drop_column(Appointment::Notes)
                    .drop_column(Appointment::Cost)
                    .drop_column(Appointment::ScheduledBy)
                    .drop_column(Appointment::AppointmentType)
                    .drop_column(Appointment::Status)
                    .add_column(ColumnDef::new(Alias::new("staff_id")).uuid().not_null())
                    .modify_column(ColumnDef::new(Appointment::PatientId).integer().not_null())
                    .modify_column(ColumnDef::new(Appointment::DoctorId).integer().not_null())
                    .modify_column(ColumnDef::new(Appointment::HospitalId).integer().not_null())
                    .to_owned(),
            )
            .await?;
        Ok(())
    }
}