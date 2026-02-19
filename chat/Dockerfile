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

COPY chat/Cargo.toml /app/chat/Cargo.toml
COPY chat/build.rs /app/chat/build.rs
COPY chat/proto /app/chat/proto
COPY chat/src /app/chat/src

COPY common/Cargo.toml /app/common/Cargo.toml
COPY common/src /app/common/src

WORKDIR /app/chat

RUN cargo build --release

FROM debian:trixie-20260202-slim

RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/chat/target/release/idklol-chat /usr/local/bin/idklol-chat

EXPOSE 50052

ENTRYPOINT ["idklol-chat"]
