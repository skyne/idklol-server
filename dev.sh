#!/bin/bash
# Start development environment with hot reload

set -e

echo "Starting development environment..."
echo "Services will auto-rebuild on code changes"
echo ""

docker compose -f docker-compose.dev.yml up --build
