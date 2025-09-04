# Build stage
FROM rust:1.80-slim as builder

WORKDIR /app

# Install system dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy manifests first for better layer caching
COPY Cargo.toml ./

# Create a minimal main.rs to satisfy initial dependency build
RUN mkdir src && echo "fn main() {}" > src/main.rs

# Build dependencies first (this layer will be cached)
RUN cargo build --release
RUN rm src/main.rs

# Copy source code
COPY src ./src

# Build the application (only our code needs recompiling)
RUN touch src/main.rs && cargo build --release

# Runtime stage
FROM debian:bookworm-slim

WORKDIR /app

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    sqlite3 \
    && rm -rf /var/lib/apt/lists/*

# Copy the binary from the builder stage
COPY --from=builder /app/target/release/demo-service /app/demo-service

# Create directory for database
RUN mkdir -p /app/data

# Expose port
EXPOSE 3000

# Set environment variables
ENV DATABASE_URL=sqlite:/app/data/products.db
ENV RUST_LOG=info

# Run the binary
CMD ["./demo-service"]