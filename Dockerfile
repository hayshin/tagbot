# # Use musl Rust image
FROM clux/muslrust:1.94.0-stable AS builder
WORKDIR /usr/src/tagbot

# Keep a consistent target dir for all builds in this stage
ENV CARGO_TARGET_DIR=/volume/target

# Copy the Cargo.toml and Cargo.lock files to leverage Docker's caching mechanism
COPY Cargo.toml Cargo.lock ./

# Build the dependencies of the application separately
RUN --mount=type=cache,target=/root/.cargo/registry \
    --mount=type=cache,target=/volume/target \
    mkdir src \
    && echo "fn main() {}" > src/main.rs \
    && cargo build --release \
    && rm -r src

COPY src ./src
RUN --mount=type=cache,target=/root/.cargo/registry \
    --mount=type=cache,target=/volume/target \
    cargo install --bin tagbot --path=.

FROM scratch
WORKDIR /app

COPY --from=builder /opt/cargo/bin/tagbot .
USER 1000

CMD ["/app/tagbot"]
