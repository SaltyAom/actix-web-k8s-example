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

# --- Up and running ---
FROM alpine:3.13

COPY --from=builder /usr/local/cargo/bin/hello-actix .
# COPY static ./static

CMD ["./hello-actix"]
EXPOSE 8080