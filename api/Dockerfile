# Stage 1: Build
FROM rust:latest AS builder

RUN rustup target add x86_64-unknown-linux-musl

WORKDIR /api
COPY . .
RUN cargo build --release --target x86_64-unknown-linux-musl

# Stage 2: Runtime
FROM debian:bullseye-slim

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /api/target/x86_64-unknown-linux-musl/release/api /usr/local/bin/api

EXPOSE 8080
CMD ["api"]
