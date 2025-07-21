use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table("appointment")
                    .modify_column(ColumnDef::new("patient_id").uuid().not_null())
                    .modify_column(ColumnDef::new("doctor_id").uuid().not_null())
                    .modify_column(ColumnDef::new("hospital_id").uuid().not_null())
                    .to_owned(),
            )
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table("appointment")
                    .modify_column(ColumnDef::new("patient_id").integer().not_null())
                    .modify_column(ColumnDef::new("doctor_id").integer().not_null())
                    .modify_column(ColumnDef::new("hospital_id").integer().not_null())
                    .to_owned(),
            )
            .await?;
        Ok(())
    }
}
