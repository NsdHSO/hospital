use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(UserProfile::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(UserProfile::UserSub)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(UserProfile::SchemaVersion)
                            .integer()
                            .not_null()
                            .default(1),
                    )
                    .col(
                        ColumnDef::new(UserProfile::AttributesJson)
                            .json_binary()
                            .not_null(),
                    )
                    .col(
ColumnDef::new(UserProfile::UpdatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::cust("now()")),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(UserProfile::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum UserProfile {
    Table,
    UserSub,
    SchemaVersion,
    AttributesJson,
    UpdatedAt,
}
