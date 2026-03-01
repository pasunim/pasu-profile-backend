# Build stage
FROM rust:1-slim-bookworm AS builder

# Install required system dependencies for building
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    curl \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the source code
COPY Cargo.toml ./
COPY src ./src

# Build the application in release mode
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies (OpenSSL is often needed for reqwest/sqlx TLS)
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the compiled binary from the builder stage
COPY --from=builder /app/target/release/pasu-profile-backend ./

# Copy the example environment file (optional, but good for reference)
COPY .env.example ./

# Expose the API port
EXPOSE 8080

# Command to run the application
CMD ["./pasu-profile-backend"]
