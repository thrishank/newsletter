#!/bin/bash

set -e

IMAGE="registry.digitalocean.com/email-newsletter/email:latest"
CONTAINER_NAME="email_newsletter"

echo "Pulling latest image..."
docker pull "$IMAGE"

echo "Backup the sql database"
export DATABASE_URL=postgresql://postgres:mysecretpassword@localhost:5432/email_newsletter
pg_dump "$DATABASE_URL" >"backup_$(date +%Y%m%d_%H%M%S).sql"

echo "Running migrations"
/root/.cargo/bin/sqlx migrate run --source /root/migrations --database-url $DATABASE_URL

echo "Remove migraions folder"
rm -rf migrations/

# Stop and remove the container only if it exists
if docker ps -a --format '{{.Names}}' | grep -Eq "^${CONTAINER_NAME}$"; then
  echo "Stopping and removing old container..."
  docker stop "$CONTAINER_NAME"
  docker rm "$CONTAINER_NAME"
else
  echo "No existing container named $CONTAINER_NAME found. Skipping stop/remove."
fi

echo "Starting new container..."
docker run -d --name $CONTAINER_NAME -p 8000:3000 --restart always "$IMAGE"

echo "Deployment complete."
