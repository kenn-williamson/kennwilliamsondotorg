# SQLx CLI container for database migrations using our setup-db.sh script
FROM rust:1.89-alpine

# Install necessary packages for building SQLx and running bash scripts
RUN apk add --no-cache \
    musl-dev \
    postgresql-client \
    bash \
    grep \
    sed \
    coreutils

# Install SQLx CLI 0.8.6 (latest version)
RUN cargo install sqlx-cli --version 0.8.6 --no-default-features --features postgres

# Set working directory
WORKDIR /app

# Copy our migration script and make it executable
COPY scripts/setup-db.sh /app/setup-db.sh
RUN chmod +x /app/setup-db.sh

# Copy migrations directory (will be overridden by volume mount)
COPY backend/migrations /app/backend/migrations

# Set environment variables that the script expects
ENV PROJECT_ROOT=/app
ENV BACKEND_DIR=/app/backend

# Default command handles SKIP_MIGRATIONS logic and runs our setup script
CMD ["sh", "-c", "if [ \"$SKIP_MIGRATIONS\" = \"true\" ]; then echo \"Skipping migrations (SKIP_MIGRATIONS=true)\"; exit 0; else echo \"Running database migrations via setup-db.sh...\"; /app/setup-db.sh; fi"]