use sea_orm_migration::prelude::*;
use ::sea_orm::Statement;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        // 1) Helpful index for joins
        db.execute(Statement::from_string(
            manager.get_database_backend(),
            "CREATE INDEX IF NOT EXISTS idx_auth_identity_person_id ON auth_identity(person_id);",
        ))
        .await?;

        // 2) Backfill from staff via auth_identity mapping
        db.execute(Statement::from_string(
            manager.get_database_backend(),
            r#"
            INSERT INTO user_profile (user_sub, schema_version, attributes_json, updated_at)
            SELECT ai.user_sub,
                   1,
                   jsonb_build_object(
                     'hospital_id', s.hospital_id::text,
                     'department_id', s.department_id::text,
                     'role', s.role::text,
                     'on_call', false
                   )::jsonb,
                   now()
            FROM auth_identity ai
            JOIN staff s ON s.id = ai.person_id
            ON CONFLICT (user_sub) DO NOTHING;
            "#,
        ))
        .await?;

        // 3) Fallback: if exactly one hospital exists, default for mappings with no staff
        db.execute(Statement::from_string(
            manager.get_database_backend(),
            r#"
            INSERT INTO user_profile (user_sub, schema_version, attributes_json, updated_at)
            SELECT ai.user_sub,
                   1,
                   jsonb_build_object(
                     'hospital_id', (SELECT id::text FROM hospital LIMIT 1),
                     'role', 'USER',
                     'on_call', false
                   )::jsonb,
                   now()
            FROM auth_identity ai
            LEFT JOIN staff s ON s.id = ai.person_id
            WHERE s.id IS NULL
              AND (SELECT COUNT(*) = 1 FROM hospital)
            ON CONFLICT (user_sub) DO NOTHING;
            "#,
        ))
        .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        // Drop the index; we do not attempt to delete backfilled data in down migration
        db.execute(Statement::from_string(
            manager.get_database_backend(),
            "DROP INDEX IF EXISTS idx_auth_identity_person_id;",
        ))
        .await?;
        Ok(())
    }
}