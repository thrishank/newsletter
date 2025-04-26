#!/bin/bash

set -x
set -eo pipefail

if ! [ -x "$(command -v psql)" ]; then
  echo >&2 "Error: psql is not installed."
  exit 1
fi

if ! [ -x "$(command -v sqlx)" ]; then
  echo >&2 "Error: sqlx is not installed."
  echo >&2 "Use:"
  echo >&2 " echo >&2 "to install it."
  exit 1
  cargo install --version=0.5.7 sqlx-cli --no-default-features --features postgres"
fi

# Variables
CONTAINER_NAME="postgres"
POSTGRES_PASSWORD="mysecretpassword"
POSTGRES_USER="postgres"
POSTGRES_DB="email_newsletter"
PORT="5432"
HOST="localhost"

# Pull latest postgres image
# docker pull postgres:latest

if [[-z "$SKIP_DOCKER"]]; then

  # Remove existing container if any
  docker rm -f $CONTAINER_NAME 2>/dev/null

  # Run postgres container
  docker run -d \
    --name $CONTAINER_NAME \
    -e POSTGRES_PASSWORD=$POSTGRES_PASSWORD \
    -e POSTGRES_USER=$POSTGRES_USER \
    -e POSTGRES_DB=$POSTGRES_DB \
    -p $PORT:5432 \
    postgres:latest
fi
# Echo connection URL
CONNECTION_URL="postgresql://${POSTGRES_USER}:${POSTGRES_PASSWORD}@${HOST}:${PORT}/${POSTGRES_DB}"
echo "PostgreSQL is starting..."
echo "Connection URL: $CONNECTION_URL"
export DATABASE_URL=$CONNECTION_URL

until psql -h "$HOST" -U "$POSTGRES_USER" -p "$PORT" -d "postgres" -c '\q'; do
  >&2 echo "Postgres is still unavailable - sleeping"
  sleep 1
done

sqlx database create
