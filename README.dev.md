# Development Mode with Hot Reload

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
