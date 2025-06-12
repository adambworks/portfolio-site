#!/bin/bash

# Exit on error
set -e

# Optional: Change to the directory of the script
cd "$(dirname "$0")"

echo "Stopping and removing existing containers..."
docker compose down

echo "Pulling latest changes from git..."
git pull origin main  # or replace 'main' with your branch name

echo "Rebuilding and starting containers..."
docker compose up --build -d

echo "✅ Deployment complete."