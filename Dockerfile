FROM clux/muslrust:1.94.0-stable AS builder
WORKDIR /usr/src

RUN USER=root cargo new tagbot
WORKDIR /usr/src/tagbot
COPY Cargo.toml Cargo.lock ./
RUN cargo build --release

COPY src ./src
RUN --mount=type=cache,target=/volume/target \
    --mount=type=cache,target=/root/.cargo/registry \
    cargo install --bin tagbot --path=.

FROM scratch
COPY --from=builder /opt/cargo/bin/tagbot /app/
USER 1000
CMD ["/app/tagbot"]
