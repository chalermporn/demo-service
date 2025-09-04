# Demo Service - Deployment Guide

This guide explains how to deploy the hexagonal architecture demo service.

## Prerequisites

- Docker & Docker Compose (for containerized deployment)
- Rust 1.81+ (for local deployment)

## Deployment Options

### Option 1: Local Development Deployment

**Recommended for development and testing:**

```bash
# Install dependencies and build
cargo build --release

# Run with default settings
cargo run

# Or run with custom database location
DATABASE_URL="sqlite:./data/products.db" cargo run

# Run in background
nohup cargo run > service.log 2>&1 &
```

### Option 2: Docker Deployment (Simple)

**For containerized deployment:**

1. **Build the application locally first:**
   ```bash
   cargo build --release
   ```

2. **Deploy with simple Docker setup:**
   ```bash
   # Build local binary and deploy
   docker compose -f docker-compose.simple.yml up --build -d
   ```

   **Note:** This approach builds the binary on the host and copies it to the container.

### Option 3: Docker Issues & Solutions

**Current Docker Build Challenge:**

The Rust ecosystem has moved to newer crates requiring edition2024 features (like `base64ct v1.8.0`), which aren't stable in Rust versions 1.75-1.82. This creates a dependency conflict when building in Docker containers.

**Recommended Solutions:**

1. **Use Local Development** (Option 1) - Works perfectly with current setup
2. **Use Pre-built Binary** (Option 2) - Works for same-architecture deployment
3. **Wait for Rust 1.83+** - Will include stable edition2024 support
4. **Use Nightly Rust** - For immediate Docker builds (not recommended for production)

**For Immediate Docker Deployment:**

⚠️ **Architecture Compatibility Note**: The simple Docker approach only works when building and deploying on the same architecture (Linux-to-Linux or with cross-compilation setup).

**On Linux systems:**
```bash
# This works on Linux hosts
cargo build --release
docker compose -f docker-compose.simple.yml up --build -d
```

**On macOS/Windows (requires cross-compilation):**
```bash
# Cross-compilation setup required (not included in this demo)
# Alternative: Use local development deployment (Option 1)
cargo run
```

**Status:** The hexagonal architecture and application code are complete and fully functional. Docker deployment currently faces two challenges: Rust ecosystem compatibility and cross-platform compilation.

## Configuration

### Environment Variables

- `DATABASE_URL`: SQLite database path (default: `sqlite:/app/data/products.db`)
- `RUST_LOG`: Log level (default: `info`)

### Ports

- **3000**: Direct service access
- **80**: Nginx proxy access (with proxy profile)

## Data Persistence

The SQLite database is persisted in the `./data` directory which is mounted as a Docker volume.

## API Endpoints

- `GET /products` - List all products
- `POST /products` - Create a product
- `GET /products/:id` - Get a product by ID
- `PUT /products/:id` - Update a product
- `DELETE /products/:id` - Delete a product
- `GET /swagger-ui` - API documentation
- `GET /api-docs/openapi.json` - OpenAPI schema

## Health Checks

The service includes a health check that monitors the `/products` endpoint every 30 seconds.

## Production Considerations

1. **Security**: Use proper secrets management for sensitive data
2. **Database**: Consider using PostgreSQL for production workloads
3. **Monitoring**: Add application metrics and logging
4. **Scaling**: Use multiple replicas behind a load balancer
5. **SSL/TLS**: Configure HTTPS with proper certificates

## Troubleshooting

1. **Check logs**:
   ```bash
   docker-compose logs demo-service
   ```

2. **Health status**:
   ```bash
   docker-compose ps
   ```

3. **Database access**:
   ```bash
   docker-compose exec demo-service sqlite3 /app/data/products.db
   ```