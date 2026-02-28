# syntax=docker/dockerfile:1

FROM rust:1.93-slim AS builder

RUN apt-get update \
    && apt-get install -y --no-install-recommends \
        ca-certificates \
        protobuf-compiler \
        libprotobuf-dev \
        pkg-config \
        libssl-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY characters/Cargo.toml /app/characters/Cargo.toml
COPY characters/build.rs /app/characters/build.rs
COPY characters/migrations /app/characters/migrations
COPY characters/proto /app/characters/proto
COPY characters/src /app/characters/src

COPY common/Cargo.toml /app/common/Cargo.toml
COPY common/src /app/common/src

WORKDIR /app/characters

# Build argument to specify which binary to build
ARG BINARY_NAME=characters-grpc

RUN cargo build --release --bin ${BINARY_NAME}

FROM debian:trixie-20260202-slim

RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Use build argument to copy the correct binary
ARG BINARY_NAME=characters-grpc
COPY --from=builder /app/characters/target/release/${BINARY_NAME} /usr/local/bin/service

EXPOSE 50052

ENTRYPOINT ["service"]
