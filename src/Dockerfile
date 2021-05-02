# --- Build Stage ---
FROM rust:1.51-alpine AS builder
WORKDIR /usr/src/
RUN apk add --no-cache musl-dev ca-certificates cmake musl-utils libressl-dev

# Setup tools for building
RUN rustup target add x86_64-unknown-linux-musl
# RUN apk add --no-cache musl-dev ca-certificates cmake musl-dev libressl-dev

# Create dummy project for package installation caching
RUN USER=root cargo new hello-actix
WORKDIR /usr/src/hello-actix

COPY Cargo.toml Cargo.lock ./

RUN cargo build --release

# Build project
COPY src ./src
# COPY static ./static

ENV PKG_CONFIG_ALLOW_CROSS=1

RUN cargo install --target x86_64-unknown-linux-musl --path .

# --- Compression ---
FROM alpine:3.13 AS compressor
WORKDIR /usr/app

RUN apk add --no-cache upx

COPY --from=builder /usr/local/cargo/bin/hello-actix .

RUN upx hello-actix -9 -o app

# --- Up and running ---
# FROM alpine:3.13
FROM scratch

COPY --from=compressor /usr/app/app .
# COPY static ./static

EXPOSE 8080

CMD ["./app"]