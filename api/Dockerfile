FROM rust:slim AS builder
RUN apt-get update && apt-get install -y libssl-dev libpq-dev pkg-config

COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y --no-install-recommends \
    libssl3 \
    libpq5 \
 && rm -rf /var/lib/apt/lists/*

COPY --from=builder /target/release/api /usr/local/bin/api

CMD ["/usr/local/bin/api"]