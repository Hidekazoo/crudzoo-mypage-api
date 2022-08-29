FROM rust:1.61.0 AS builder
WORKDIR /app
COPY . .
ENV SQLX_OFFLINE true
ENV RUST_BACKTRACE short
ENV CARGO_INCREMENTAL 0
ENV CARGO_NET_RETRY 10
ENV RUSTUP_MAX_RETRIES 10
RUN cargo build -p api-axum --release

FROM debian:buster-slim AS runtime
WORKDIR /app
RUN apt-get update -y \
    && apt-get install -y --no-install-recommends openssl \
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/api crudzoo-mypage-api
COPY /api/configuration /app/api/configuration
ENV APP_ENVIRONMENT production
ENTRYPOINT [ "./crudzoo-mypage-api" ]