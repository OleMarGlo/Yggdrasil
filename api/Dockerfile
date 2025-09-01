FROM rust:slim AS builder
RUN apt-get update && apt-get install -y libssl-dev libpq-dev pkg-config

WORKDIR /api
COPY . .
RUN cargo build --release

FROM gcr.io/distroless/cc
COPY --from=builder /api/target/release/api /usr/local/bin/api
CMD ["/usr/local/bin/api"]
