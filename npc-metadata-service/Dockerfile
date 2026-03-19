# syntax=docker/dockerfile:1

FROM rust:1.93-slim AS builder

RUN apt-get update \
    && apt-get install -y --no-install-recommends \
        ca-certificates \
        pkg-config \
        libssl-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY npc-metadata-service/Cargo.toml /app/npc-metadata-service/Cargo.toml
COPY npc-metadata-service/migrations /app/npc-metadata-service/migrations
COPY npc-metadata-service/src /app/npc-metadata-service/src

COPY common/Cargo.toml /app/common/Cargo.toml
COPY common/src /app/common/src

WORKDIR /app/npc-metadata-service

RUN cargo build --release --bin npc-metadata-service

FROM debian:trixie-20260202-slim

RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/npc-metadata-service/target/release/npc-metadata-service /usr/local/bin/service

CMD ["/usr/local/bin/service"]
