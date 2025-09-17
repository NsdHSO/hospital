use sea_orm_migration::prelude::*;
use ::sea_orm::Statement;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        let backend = manager.get_database_backend();

        // Enable pg_trgm for trigram indexes (safe if already enabled). If not permitted, skip.
        db.execute(Statement::from_string(
            backend,
            r#"
            DO $$
            BEGIN
                BEGIN
                    CREATE EXTENSION IF NOT EXISTS pg_trgm;
                EXCEPTION
                    WHEN insufficient_privilege THEN
                        RAISE NOTICE 'Skipping CREATE EXTENSION pg_trgm due to insufficient privileges.';
                    WHEN undefined_file THEN
                        RAISE NOTICE 'Skipping CREATE EXTENSION pg_trgm; extension is not available on server.';
                    WHEN others THEN
                        RAISE NOTICE 'Skipping CREATE EXTENSION pg_trgm due to unexpected error: %', SQLERRM;
                END;
            END $$;
            "#.to_string(),
        ))
        .await?;

        // Add a generated tsvector column for full-text search across key person fields
        db.execute(Statement::from_string(
            backend,
            r#"
            DO $$
            BEGIN
                IF NOT EXISTS (
                    SELECT 1
                    FROM information_schema.columns
                    WHERE table_schema = 'public'
                      AND table_name = 'person'
                      AND column_name = 'search_tsv'
                ) THEN
                    ALTER TABLE public.person
                      ADD COLUMN search_tsv tsvector GENERATED ALWAYS AS (
                        setweight(to_tsvector('simple', coalesce(first_name, '')), 'A') ||
                        setweight(to_tsvector('simple', coalesce(last_name,  '')), 'A') ||
                        setweight(to_tsvector('simple', coalesce(email,      '')), 'B') ||
                        setweight(to_tsvector('simple', coalesce(phone,      '')), 'B') ||
                        setweight(to_tsvector('simple', coalesce(address,    '')), 'C')
                      ) STORED;
                END IF;
            END $$;
            "#.to_string(),
        ))
        .await?;

        // Create GIN index for the tsvector column
        db.execute(Statement::from_string(
            backend,
            r#"
            DO $$
            BEGIN
                IF NOT EXISTS (
                    SELECT 1 FROM pg_indexes
                    WHERE schemaname = 'public' AND indexname = 'idx_person_search_tsv'
                ) THEN
                    CREATE INDEX idx_person_search_tsv ON public.person USING gin (search_tsv);
                END IF;
            END $$;
            "#.to_string(),
        ))
        .await?;

        // Create trigram GIN indexes for fuzzy/substring search on common fields
        // Only if pg_trgm and the gin_trgm_ops operator class are available. If not, skip gracefully.
        db.execute(Statement::from_string(
            backend,
            r#"
            DO $$
            DECLARE
                has_trgm boolean := false;
                has_gin_trgm boolean := false;
            BEGIN
                SELECT EXISTS (SELECT 1 FROM pg_extension WHERE extname = 'pg_trgm') INTO has_trgm;
                SELECT EXISTS (
                    SELECT 1
                    FROM pg_opclass oc
                    JOIN pg_am am ON am.oid = oc.opcmethod
                    WHERE am.amname = 'gin' AND oc.opcname = 'gin_trgm_ops'
                ) INTO has_gin_trgm;

                IF has_trgm AND has_gin_trgm THEN
                    BEGIN
                        IF NOT EXISTS (SELECT 1 FROM pg_indexes WHERE schemaname='public' AND indexname='idx_person_first_name_trgm') THEN
                            CREATE INDEX idx_person_first_name_trgm ON public.person USING gin (lower(first_name) gin_trgm_ops);
                        END IF;
                        IF NOT EXISTS (SELECT 1 FROM pg_indexes WHERE schemaname='public' AND indexname='idx_person_last_name_trgm') THEN
                            CREATE INDEX idx_person_last_name_trgm ON public.person USING gin (lower(last_name) gin_trgm_ops);
                        END IF;
                        IF NOT EXISTS (SELECT 1 FROM pg_indexes WHERE schemaname='public' AND indexname='idx_person_email_trgm') THEN
                            CREATE INDEX idx_person_email_trgm ON public.person USING gin (lower(email) gin_trgm_ops);
                        END IF;
                        IF NOT EXISTS (SELECT 1 FROM pg_indexes WHERE schemaname='public' AND indexname='idx_person_phone_trgm') THEN
                            CREATE INDEX idx_person_phone_trgm ON public.person USING gin (lower(phone) gin_trgm_ops);
                        END IF;
                        IF NOT EXISTS (SELECT 1 FROM pg_indexes WHERE schemaname='public' AND indexname='idx_person_address_trgm') THEN
                            CREATE INDEX idx_person_address_trgm ON public.person USING gin (lower(address) gin_trgm_ops);
                        END IF;
                    EXCEPTION
                        WHEN undefined_object THEN
                            RAISE NOTICE 'Skipping trigram indexes for person: %', SQLERRM;
                        WHEN others THEN
                            RAISE NOTICE 'Skipping trigram indexes for person due to unexpected error: %', SQLERRM;
                    END;
                ELSE
                    RAISE NOTICE 'pg_trgm extension/operator class not available; skipping trigram indexes for person.';
                END IF;
            END $$;
            "#.to_string(),
        ))
        .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        let backend = manager.get_database_backend();

        // Drop indexes then the column. We do not drop the pg_trgm extension.
        db.execute(Statement::from_string(
            backend,
            r#"
            DO $$
            BEGIN
                -- Drop indexes if they exist
                IF EXISTS (SELECT 1 FROM pg_indexes WHERE schemaname='public' AND indexname='idx_person_search_tsv') THEN
                    DROP INDEX IF EXISTS public.idx_person_search_tsv;
                END IF;
                IF EXISTS (SELECT 1 FROM pg_indexes WHERE schemaname='public' AND indexname='idx_person_first_name_trgm') THEN
                    DROP INDEX IF EXISTS public.idx_person_first_name_trgm;
                END IF;
                IF EXISTS (SELECT 1 FROM pg_indexes WHERE schemaname='public' AND indexname='idx_person_last_name_trgm') THEN
                    DROP INDEX IF EXISTS public.idx_person_last_name_trgm;
                END IF;
                IF EXISTS (SELECT 1 FROM pg_indexes WHERE schemaname='public' AND indexname='idx_person_email_trgm') THEN
                    DROP INDEX IF EXISTS public.idx_person_email_trgm;
                END IF;
                IF EXISTS (SELECT 1 FROM pg_indexes WHERE schemaname='public' AND indexname='idx_person_phone_trgm') THEN
                    DROP INDEX IF EXISTS public.idx_person_phone_trgm;
                END IF;
                IF EXISTS (SELECT 1 FROM pg_indexes WHERE schemaname='public' AND indexname='idx_person_address_trgm') THEN
                    DROP INDEX IF EXISTS public.idx_person_address_trgm;
                END IF;

                -- Drop column if exists
                IF EXISTS (
                    SELECT 1 FROM information_schema.columns
                    WHERE table_schema = 'public' AND table_name = 'person' AND column_name = 'search_tsv'
                ) THEN
                    ALTER TABLE public.person DROP COLUMN IF EXISTS search_tsv;
                END IF;
            END $$;
            "#.to_string(),
        ))
        .await?;

        Ok(())
    }
}
