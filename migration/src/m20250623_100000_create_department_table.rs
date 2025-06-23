use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table("department")
                    .add_column_if_not_exists(ColumnDef::new("created_at").date_time().not_null())
                    .add_column_if_not_exists(ColumnDef::new("updated_at").date_time().not_null())
                    .add_column_if_not_exists(ColumnDef::new("hospital_id").uuid().not_null())
                    .add_column_if_not_exists(ColumnDef::new("floor").integer().null())
                    .add_column_if_not_exists(ColumnDef::new("head_of_department").string().null())
                    .add_column_if_not_exists(ColumnDef::new("phone").string().null())
                    .add_column_if_not_exists(ColumnDef::new("description").string().null())
                    .add_column_if_not_exists(ColumnDef::new("capacity").integer().null())
                    .add_column_if_not_exists(ColumnDef::new("name").string().not_null())
                    .add_column_if_not_exists(ColumnDef::new("department_ic").string().null())
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .name("idx-unique-department-name")
                    .table("department")
                    .col("name")
                    .unique()
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_index(
                Index::drop()
                    .name("idx-unique-department-name")
                    .table("department")
                    .to_owned(),
            )
            .await?;
        // Optionally, drop columns (not recommended in production)
        Ok(())
    }
}
