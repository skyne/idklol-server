# Development Mode with Hot Reload

## Developer Docs

- NPC system overview + module map: `docs/developer/NPC_SYSTEM_DEVELOPER_GUIDE.md`
- NPC asset/new NPC authoring guide (UE side): `../idklol-client/docs/npc/NPC_ASSET_AND_CREATION_GUIDE.md`

## Quick Start

Start services with auto-reload on code changes:

**macOS/Linux:**
```bash
./dev.sh
```

**Windows:**
```cmd
dev.cmd
```

**Or manually:**
```bash
docker compose -f docker-compose.dev.yml up --build
```

## Optional Observability Stack

An opt-in profile is available for logs, traces, and metrics:
- **Grafana** (UI)
- **Loki + Promtail** (log shipping/storage)
- **Tempo + OTel Collector** (trace ingest/storage)
- **Prometheus** (metrics scraping)

Start app + observability together:

```bash
docker compose -f docker-compose.dev.yml --profile observability up --build
```

Endpoints:
- Grafana: http://localhost:3001 (admin/admin)
- Loki API: http://localhost:3100
- Tempo API: http://localhost:3200
- Prometheus: http://localhost:9090
- OTel Collector OTLP: grpc://localhost:4317 and http://localhost:4318

Notes:
- Logs are available immediately through Promtail -> Loki.
- Rust services now export traces to OTLP by default in compose via `TRACING_OTLP_ENABLED=true` and `OTEL_EXPORTER_OTLP_ENDPOINT=http://otel-collector:4317`.
- If you run without the observability profile and want to silence OTLP exporter errors, set `TRACING_OTLP_ENABLED=false` for those services.

## How It Works

- **Source mounted as volumes**: Your local code is mounted into containers
- **cargo-watch**: Automatically rebuilds and restarts on file changes
- **Cached dependencies**: Build artifacts cached in Docker volumes for speed
- **Larger containers**: Include Rust toolchain and cargo-watch

## First Build

The first build will be slow (5-10 minutes) as it:
1. Downloads Rust toolchain
2. Installs cargo-watch
3. Compiles all dependencies

## Subsequent Changes

After first build, changes trigger fast incremental rebuilds:
- **Code changes**: ~5-15 seconds
- **Dependency changes**: Slower, but still faster than full rebuild

## Watching Logs

```bash
# All services
docker compose -f docker-compose.dev.yml logs -f

# Specific service
docker compose -f docker-compose.dev.yml logs -f characterserver

# Just see rebuilds
docker compose -f docker-compose.dev.yml logs -f characterserver | grep -i compiling
```

## Production Build

For production builds (smaller images), use the regular compose file:

```bash
docker compose -f docker-compose.yml up --build -d
```

## Tips

- Edit code normally in your editor/IDE
- Container auto-detects changes and rebuilds
- No need to restart containers after code changes
- Ctrl+C to stop all services
- Use `docker compose -f docker-compose.dev.yml down` to clean up

## Troubleshooting

**Rebuild not triggering?**
- Check file was saved
- Look for cargo-watch output in logs

**Very slow builds?**
- First build is always slow
- Make sure volumes are being used (check docker-compose.dev.yml)
- On Mac, ensure `:cached` is on source mount

**Want to rebuild from scratch?**
```bash
docker compose -f docker-compose.dev.yml down -v
docker compose -f docker-compose.dev.yml up --build
```
