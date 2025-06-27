use sea_orm::sea_query::extension::postgres::Type;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;
use sea_orm::{ConnectionTrait, Statement};

async fn enum_type_exists(manager: &SchemaManager<'_>, enum_name: &str) -> Result<bool, DbErr> {
    let db = manager.get_connection();
    let check_query = format!(
        "SELECT EXISTS (SELECT 1 FROM pg_type WHERE typname = '{}');",
        enum_name
    );
    let result = db.query_one(Statement::from_string(
        db.get_database_backend(),
        check_query,
    )).await?;

    if let Some(row) = result {
        let exists: bool = row.try_get("", "exists")?;
        Ok(exists)
    } else {
        Ok(false)
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create ENUM type
        if !enum_type_exists(manager, "department_name_enum").await? {
            manager
                .create_type(
                    Type::create()
                        .as_enum("department_name_enum")
                        .values([
                            "CARDIOLOGY",
                            "ONCOLOGY",
                            "NEUROLOGY",
                            "PEDIATRICS",
                            "SURGERY",
                            "INTERNAL_MEDICINE",
                            "OBSTETRICS_GYNECOLOGY",
                            "OPHTHALMOLOGY",
                            "DERMATOLOGY",
                            "UROLOGY",
                        ])
                        .to_owned(),
                )
                .await?;
        }

        // Alter table
        manager
            .alter_table(
                Table::alter()
                    .table("department")
                    .add_column_if_not_exists(ColumnDef::new("created_at").timestamp().not_null())
                    .add_column_if_not_exists(ColumnDef::new("updated_at").timestamp().not_null())
                    .add_column_if_not_exists(ColumnDef::new("hospital_id").uuid().not_null())
                    .add_column_if_not_exists(ColumnDef::new("floor").integer().null())
                    .add_column_if_not_exists(ColumnDef::new("head_of_department").string().null())
                    .add_column_if_not_exists(ColumnDef::new("phone").string().null())
                    .add_column_if_not_exists(ColumnDef::new("description").text().null())
                    .add_column_if_not_exists(ColumnDef::new("capacity").integer().null())
                    .add_column_if_not_exists(
                        ColumnDef::new("name")
                            .enumeration(
                                "department_name_enum",
                                [
                                    "CARDIOLOGY",
                                    "ONCOLOGY",
                                    "NEUROLOGY",
                                    "PEDIATRICS",
                                    "SURGERY",
                                    "INTERNAL_MEDICINE",
                                    "OBSTETRICS_GYNECOLOGY",
                                    "OPHTHALMOLOGY",
                                    "DERMATOLOGY",
                                    "UROLOGY",
                                ],
                            )
                            .not_null(),
                    )
                    .add_column_if_not_exists(ColumnDef::new("department_ic").string().null())
                    .to_owned(),
            )
            .await?;

        // Unique index on (hospital_id, name)
        manager
            .create_index(
                Index::create()
                    .name("uq_department_hospital_id_name")
                    .table("department")
                    .col("hospital_id")
                    .col("name")
                    .unique()
                    .if_not_exists()
                    .to_owned(),
            )
            .await?;

        // Index on hospital_id
        manager
            .create_index(
                Index::create()
                    .name("idx_department_hospital_id")
                    .table("department")
                    .col("hospital_id")
                    .if_not_exists()
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Drop indexes
        manager
            .drop_index(
                Index::drop()
                    .name("uq_department_hospital_id_name")
                    .table("department")
                    .if_exists()
                    .to_owned(),
            )
            .await?;

        manager
            .drop_index(
                Index::drop()
                    .name("idx_department_hospital_id")
                    .table("department")
                    .if_exists()
                    .to_owned(),
            )
            .await?;

        // Drop the added columns
        manager
            .alter_table(
                Table::alter()
                    .table("department")
                    .drop_column("department_ic")
                    .drop_column("name")
                    .drop_column("capacity")
                    .drop_column("description")
                    .drop_column("phone")
                    .drop_column("head_of_department")
                    .drop_column("floor")
                    .drop_column("hospital_id")
                    .drop_column("updated_at")
                    .drop_column("created_at")
                    .to_owned(),
            )
            .await?;

        // Drop ENUM type
        let _ = manager
            .drop_type(Type::drop().name("department_name_enum").to_owned())
            .await;

        Ok(())
    }
}
