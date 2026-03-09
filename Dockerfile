# # Use musl Rust image
FROM clux/muslrust:1.94.0-stable AS builder
WORKDIR /app

# Keep a consistent target dir for all builds in this stage
ENV CARGO_TARGET_DIR=/app/target
ENV CARGO_HOME=/app/cargo

# Copy the Cargo.toml and Cargo.lock files to leverage Docker's caching mechanism
COPY Cargo.toml Cargo.lock ./
# Build the dependencies of the application separately
RUN --mount=type=cache,target=/app/cargo/registry \
    --mount=type=cache,target=/app/target \
    cargo fetch

COPY src ./src
# Build the application with specific path
RUN --mount=type=cache,target=/app/cargo/registry \
    --mount=type=cache,target=/app/target \
    cargo install --bin tagbot --path=.

FROM scratch
COPY --from=builder /app/cargo/bin/tagbot /app/tagbot
USER 1000
CMD ["/app/tagbot"]
