# syntax=docker/dockerfile:1

FROM rust:1.93-slim AS builder

RUN apt-get update \
    && apt-get install -y --no-install-recommends \
        ca-certificates \
        pkg-config \
        libssl-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY npc-interactions-bridge/Cargo.toml /app/npc-interactions-bridge/Cargo.toml
COPY npc-interactions-bridge/src /app/npc-interactions-bridge/src

COPY common/Cargo.toml /app/common/Cargo.toml
COPY common/src /app/common/src

WORKDIR /app/npc-interactions-bridge

RUN cargo build --release --bin npc-interactions-bridge

FROM debian:trixie-20260202-slim

RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/npc-interactions-bridge/target/release/npc-interactions-bridge /usr/local/bin/npc-interactions-bridge
COPY npc-interactions-bridge/prompts /srv/prompts

WORKDIR /srv

ENTRYPOINT ["npc-interactions-bridge"]