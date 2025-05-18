#!/bin/sh
set -e

# Extract admin database connection information from environment variables
DB_USER=$(echo $DATABASE_URL | sed 's|^.*://\([^:]*\):.*|\1|')
DB_PASSWORD=$(echo $DATABASE_URL | sed 's|^.*://[^:]*:\([^@]*\)@.*|\1|')
DB_HOST=$(echo $DATABASE_URL | sed 's|^.*@\([^:/]*\).*|\1|')
DB_PORT=$(echo $DATABASE_URL | sed 's|^.*@[^:]*:\([0-9]*\)/.*|\1|')
DB_NAME=$(echo $DATABASE_URL | sed 's|^.*/\([^?]*\).*|\1|')

# Set default port if not specified
if [ -z "$DB_PORT" ]; then
  DB_PORT=5432
fi

# Get application user password from environment variable
if [ -z "$APPLICATION_PASSWORD" ]; then
  echo "ERROR: Environment variable APPLICATION_PASSWORD is not set"
  echo "Please specify a password for the application user"
  exit 1
fi
APP_PASSWORD=${APPLICATION_PASSWORD}

# Wait for database server to start
until PGPASSWORD=$DB_PASSWORD psql -h $DB_HOST -p $DB_PORT -U $DB_USER -d postgres -c '\q'; do
  echo "Database server is not ready yet. Retrying..."
  sleep 1
done

# Check if database exists
if PGPASSWORD=$DB_PASSWORD psql -h $DB_HOST -p $DB_PORT -U $DB_USER -d postgres -lqt | cut -d \| -f 1 | grep -qw $DB_NAME; then
  echo "Database '$DB_NAME' already exists."
else
  echo "Creating database '$DB_NAME'..."
  PGPASSWORD=$DB_PASSWORD psql -h $DB_HOST -p $DB_PORT -U $DB_USER -d postgres -c "CREATE DATABASE $DB_NAME;"
  echo "Database '$DB_NAME' has been created."
fi

# Check if 'application' user exists
USER_EXISTS=$(PGPASSWORD=$DB_PASSWORD psql -h $DB_HOST -p $DB_PORT -U $DB_USER -d postgres -t -c "SELECT 1 FROM pg_roles WHERE rolname='application'")
if [ -n "$USER_EXISTS" ]; then
  echo "Database user 'application' already exists."
else
  echo "Creating database user 'application'..."
  PGPASSWORD=$DB_PASSWORD psql -h $DB_HOST -p $DB_PORT -U $DB_USER -d postgres -c "CREATE USER application WITH PASSWORD '$APP_PASSWORD' LOGIN NOSUPERUSER NOCREATEDB NOCREATEROLE;"
  echo "Database user 'application' has been created."
fi

# Grant privileges to the application user
echo "Granting privileges to 'application' user on database '$DB_NAME'..."
PGPASSWORD=$DB_PASSWORD psql -h $DB_HOST -p $DB_PORT -U $DB_USER -d $DB_NAME -c "GRANT CONNECT ON DATABASE $DB_NAME TO application;"
PGPASSWORD=$DB_PASSWORD psql -h $DB_HOST -p $DB_PORT -U $DB_USER -d $DB_NAME -c "GRANT USAGE ON SCHEMA public TO application;"

# Grant DML privileges on all tables in public schema
echo "Granting SELECT, INSERT, UPDATE, DELETE on all tables in schema public to 'application'..."
PGPASSWORD=$DB_PASSWORD psql -h $DB_HOST -p $DB_PORT -U $DB_USER -d $DB_NAME -c "GRANT SELECT, INSERT, UPDATE, DELETE ON ALL TABLES IN SCHEMA public TO application;"
PGPASSWORD=$DB_PASSWORD psql -h $DB_HOST -p $DB_PORT -U $DB_USER -d $DB_NAME -c "ALTER DEFAULT PRIVILEGES IN SCHEMA public GRANT SELECT, INSERT, UPDATE, DELETE ON TABLES TO application;"

# Grant privileges on sequences (if your application needs them)
PGPASSWORD=$DB_PASSWORD psql -h $DB_HOST -p $DB_PORT -U $DB_USER -d $DB_NAME -c "GRANT USAGE, SELECT ON ALL SEQUENCES IN SCHEMA public TO application;"
PGPASSWORD=$DB_PASSWORD psql -h $DB_HOST -p $DB_PORT -U $DB_USER -d $DB_NAME -c "ALTER DEFAULT PRIVILEGES IN SCHEMA public GRANT USAGE, SELECT ON SEQUENCES TO application;"

# Execute the command specified by the command line arguments (migration)
echo "Executing command: $@"
exec "$@"
