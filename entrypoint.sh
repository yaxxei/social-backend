#!/bin/sh

set -ex

if [ -n "$DATABASE_URL" ] && [ -d "migrations" ]; then
  echo "Checking for pending migrations..."
  
  # Проверяем, есть ли неприменённые миграции
  if sqlx migrate info -D $DATABASE_URL | grep -q "pending"; then
    echo "Applying migrations..."
    sqlx migrate run -D $DATABASE_URL
  else
    echo "All migrations are already applied."
  fi
fi

exec "$@"