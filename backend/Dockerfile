# Multi-stage build for optimization
FROM rust:1.90-alpine AS builder

WORKDIR /app
RUN apk add --no-cache musl-dev openssl-dev

# Build dependencies first (cache layer)
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm src/main.rs

# Build application
COPY src ./src
COPY migrations ./migrations
COPY .sqlx ./.sqlx
RUN touch src/main.rs
RUN cargo build --release

# Runtime stage
FROM alpine:latest
RUN apk add --no-cache ca-certificates

WORKDIR /app

# Copy built binary
COPY --from=builder /app/target/release/backend /app/

# Copy migrations for runtime
COPY --from=builder /app/migrations /app/migrations

# Create non-root user
RUN addgroup -g 1001 -S appgroup && \
    adduser -S appuser -u 1001 -G appgroup

USER appuser

EXPOSE 8080

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD wget --no-verbose --tries=1 --spider http://localhost:8080/health || exit 1

CMD ["./backend"]