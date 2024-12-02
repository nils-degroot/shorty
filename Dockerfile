FROM rust:1-slim-bookworm AS builder

WORKDIR /app

COPY Cargo.toml Cargo.lock /app
COPY src/ /app/src
COPY migrations/ /app/migrations
COPY .sqlx/ /app/.sqlx

RUN cargo build --release

FROM debian:bookworm

COPY --from=builder /app/target/release/shorty /app

COPY static/ /static

CMD ["/app"]
