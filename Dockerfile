FROM rust:1.88-bookworm AS builder

WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY crates ./crates
COPY static ./static

RUN cargo build --release -p api

FROM debian:bookworm-slim AS runtime

RUN useradd --create-home --uid 10001 visualizador
WORKDIR /app

COPY --from=builder /app/target/release/api /usr/local/bin/visualizador-api
COPY --from=builder /app/static ./static

ENV HTTP_BIND=0.0.0.0:3000
EXPOSE 3000

USER visualizador
ENTRYPOINT ["/usr/local/bin/visualizador-api"]
