@echo off
REM Start development environment with hot reload

echo Starting development environment...
echo Services will auto-rebuild on code changes
echo.

docker compose -f docker-compose.dev.yml up --build
