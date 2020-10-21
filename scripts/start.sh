#! /bin/sh
echo "Running migrations"

sqlx migrate run

echo "Starting server"

backend