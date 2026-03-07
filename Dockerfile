# Build stage
FROM rust:1.83 AS builder

WORKDIR /app

# Copy manifests
COPY Cargo.toml Cargo.lock* ./

# Copy source code
COPY src ./src

# Build the application
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

# Install necessary dependencies
RUN apt-get update && \
    apt-get install -y ca-certificates libssl3 && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the binary from builder
COPY --from=builder /app/target/release/telegram-tag-bot /app/telegram-tag-bot

# Set environment variables (override these when running)
ENV RUST_LOG=info
ENV TELOXIDE_TOKEN=""

# Run the bot
CMD ["/app/telegram-tag-bot"]
