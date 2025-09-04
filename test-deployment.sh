#!/bin/bash

echo "=== Testing Docker Deployment Setup ==="

# Check required files exist
echo "Checking deployment files..."
files=("Dockerfile" "docker-compose.yml" ".dockerignore" "nginx.conf" "DEPLOYMENT.md")

for file in "${files[@]}"; do
    if [ -f "$file" ]; then
        echo "✅ $file exists"
    else
        echo "❌ $file missing"
    fi
done

echo ""
echo "=== Deployment Commands ==="
echo "Basic deployment:"
echo "  docker-compose up --build"
echo ""
echo "With nginx proxy:"
echo "  docker-compose --profile with-proxy up --build"
echo ""
echo "Background deployment:"
echo "  docker-compose up --build -d"
echo ""
echo "View logs:"
echo "  docker-compose logs -f demo-service"
echo ""
echo "Stop deployment:"
echo "  docker-compose down"
echo ""
echo "=== Environment Variables ==="
echo "DATABASE_URL: Path to SQLite database"
echo "RUST_LOG: Logging level (info, debug, warn, error)"
echo ""
echo "=== Ports ==="
echo "3000: Direct service access"
echo "80: Nginx proxy (with --profile with-proxy)"
echo ""
echo "=== Data Persistence ==="
echo "SQLite database persisted in ./data directory"