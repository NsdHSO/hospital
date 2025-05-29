use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table("emergency_patient")
                    .if_not_exists()
                    .col(
                        ColumnDef::new("emergency_id")
                            .uuid()
                            .not_null()
                    )
                    .col(
                        ColumnDef::new("patient_id")
                            .uuid()
                            .not_null()
                    )
                    .primary_key(
                        Index::create()
                            .col("emergency_id")
                            .col("patient_id")
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_emergency_patient_emergency")
                            .from("emergency_patient", "emergency_id")
                            .to("emergency", "id")
                            .on_delete(ForeignKeyAction::Cascade)
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_emergency_patient_patient")
                            .from("emergency_patient", "patient_id")
                            .to("patient", "id")
                            .on_delete(ForeignKeyAction::Cascade)
                    )
                    .to_owned(),
            )
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table("emergency_patient").to_owned())
            .await
    }
}
