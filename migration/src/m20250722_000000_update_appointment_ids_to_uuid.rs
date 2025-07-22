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
                    .drop_column(Alias::new("staff_id"))
                    .add_column_if_not_exists(ColumnDef::new("created_at").timestamp().not_null())
                    .add_column_if_not_exists(ColumnDef::new("updated_at").timestamp().not_null())
                    .add_column_if_not_exists(ColumnDef::new("id").uuid().not_null().primary_key())
                    .add_column_if_not_exists(ColumnDef::new("appointment_ic").integer().unique_key().not_null())
                    .add_column_if_not_exists(ColumnDef::new("patient_id").uuid().not_null())
                    .add_column_if_not_exists(ColumnDef::new("doctor_id").uuid().not_null())
                    .add_column_if_not_exists(ColumnDef::new("hospital_id").uuid().not_null())
                    .add_column_if_not_exists(ColumnDef::new("appointment_date").timestamp().not_null())
                    .add_column_if_not_exists(ColumnDef::new("reason").text().null())
                    .add_column_if_not_exists(ColumnDef::new("notes").text().null())
                    .add_column_if_not_exists(ColumnDef::new("cost").decimal().not_null())
                    .add_column_if_not_exists(ColumnDef::new("scheduled_by").string().null())
                    .add_column_if_not_exists(ColumnDef::new("appointment_type").string().null())
                    .add_column_if_not_exists(ColumnDef::new("status").string().not_null()) // You may want to use ENUM if defined
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
                    .add_column(
                        ColumnDef::new(Alias::new("staff_id"))
                            .uuid()
                            .not_null()
                    )
                    .modify_column(ColumnDef::new("patient_id").integer().not_null())
                    .modify_column(ColumnDef::new("doctor_id").integer().not_null())
                    .modify_column(ColumnDef::new("hospital_id").integer().not_null())
                    .to_owned(),
            )
            .await?;
        Ok(())
    }
}
