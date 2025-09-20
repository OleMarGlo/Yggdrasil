FROM rust:slim AS builder
RUN apt-get update && apt-get install -y libssl-dev libpq-dev pkg-config

WORKDIR /api
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y libssl-dev libpq-dev

COPY --from=builder /api/target/release/api /usr/local/bin/api

ENV RUST_LOG=info
ENV RUST_BACKTRACE=1
ENV CARGO_TERM_COLOR=always

CMD ["/usr/local/bin/api"]