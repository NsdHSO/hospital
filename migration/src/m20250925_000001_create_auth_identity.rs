use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(AuthIdentity::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(AuthIdentity::UserSub)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(AuthIdentity::PersonId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(AuthIdentity::CreatedAt)
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
            .drop_table(Table::drop().table(AuthIdentity::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum AuthIdentity {
    Table,
    UserSub,
    PersonId,
    CreatedAt,
}
