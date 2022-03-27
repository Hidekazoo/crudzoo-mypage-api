# FROM rust:1.54.0 AS builder
FROM lukemathwalker/cargo-chef:latest-rust-1.56.0 as chef
WORKDIR /app

FROM chef as planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef as builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
ENV SQLX_OFFLINE true
RUN cargo build -p api --release

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
# ENTRYPOINT ["./target/release/crudzoo-note-api"]