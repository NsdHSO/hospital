use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Unique index for patient_ic in patient
        manager
            .create_index(
                Index::create()
                    .name("uq_patient_ic")
                    .table("patient")
                    .col("patient_ic")
                    .unique()
                    .if_not_exists()
                    .to_owned(),
            )
            .await?;
        // Unique index for staff_ic in staff
        manager
            .create_index(
                Index::create()
                    .name("uq_staff_ic")
                    .table("staff")
                    .col("staff_ic")
                    .unique()
                    .if_not_exists()
                    .to_owned(),
            )
            .await?;
        // Unique index for department_ic in department
        manager
            .create_index(
                Index::create()
                    .name("uq_department_ic")
                    .table("department")
                    .col("department_ic")
                    .unique()
                    .if_not_exists()
                    .to_owned(),
            )
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Drop unique index for patient_ic
        manager
            .drop_index(
                Index::drop()
                    .name("uq_patient_ic")
                    .table("patient")
                    .if_exists()
                    .to_owned(),
            )
            .await?;
        // Drop unique index for staff_ic
        manager
            .drop_index(
                Index::drop()
                    .name("uq_staff_ic")
                    .table("staff")
                    .if_exists()
                    .to_owned(),
            )
            .await?;
        // Drop unique index for department_ic
        manager
            .drop_index(
                Index::drop()
                    .name("uq_department_ic")
                    .table("department")
                    .if_exists()
                    .to_owned(),
            )
            .await?;
        Ok(())
    }
}
