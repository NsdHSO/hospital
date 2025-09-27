#!/usr/bin/env bash
set -Eeuo pipefail

# Load DATABASE_URL from environment or .env (without overwriting an existing env var)
if [[ -z "${DATABASE_URL:-}" ]]; then
  if [[ -f ".env" ]]; then
    # shellcheck disable=SC1091
    set -a
    source .env
    set +a
  fi
fi

if [[ -z "${DATABASE_URL:-}" ]]; then
  echo "ERROR: DATABASE_URL is not set. Set it in your environment or in .env" >&2
  exit 1
fi

# Ask for schema (e.g., auth or public), default to 'auth'
read -r -p "Database schema to use [auth] (e.g., auth or public): " DB_SCHEMA
DB_SCHEMA="${DB_SCHEMA:-auth}"

echo "Running database migrations against schema: ${DB_SCHEMA} ..."
# Execute without printing the URL to avoid exposing credentials
cargo run --manifest-path migration/Cargo.toml -- --database-url "$DATABASE_URL" --database-schema "$DB_SCHEMA"

echo "Migrations completed!"
