#!/bin/sh
set -e

# Extract database connection information from environment variables
DB_USER=$(echo $DATABASE_URL | sed -n 's/.*:\/\/\([^:]*\):.*/\1/p')
DB_PASSWORD=$(echo $DATABASE_URL | sed -n 's/.*:\/\/[^:]*:\([^@]*\).*/\1/p')
DB_HOST=$(echo $DATABASE_URL | sed -n 's/.*@\([^/:]*\).*\/\(.*\)/\1/p')
DB_PORT=$(echo $DATABASE_URL | sed -n 's/.*@[^/:]*:\([0-9]*\).*\/\(.*\)/\1/p')
DB_NAME=$(echo $DATABASE_URL | sed -n 's/.*\/\([^?]*\).*/\1/p')

# Set default port if not specified
if [ -z "$DB_PORT" ]; then
  DB_PORT=5432
fi

# Wait for database server to start
echo "Waiting for database server to start..."
until PGPASSWORD=$DB_PASSWORD psql -h $DB_HOST -p $DB_PORT -U $DB_USER -c '\q'; do
  echo "Database server is not ready yet. Retrying..."
  sleep 1
done

# Check if database exists
if PGPASSWORD=$DB_PASSWORD psql -h $DB_HOST -p $DB_PORT -U $DB_USER -lqt | cut -d \| -f 1 | grep -qw $DB_NAME; then
  echo "Database '$DB_NAME' already exists."
else
  echo "Creating database '$DB_NAME'..."
  PGPASSWORD=$DB_PASSWORD psql -h $DB_HOST -p $DB_PORT -U $DB_USER -c "CREATE DATABASE $DB_NAME;"
  echo "Database '$DB_NAME' has been created."
fi

# Execute the command specified by the command line arguments (migration)
echo "Executing command: $@"
exec "$@"
