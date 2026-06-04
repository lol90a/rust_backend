# rust-backend

Production-ready Rust workspace following **Clean Architecture**.

## Workspace layout

```
crates/
  app-core/   Domain logic · Application services · Infrastructure adapters
  server/     Actix Web HTTP server
```

## Quick start

```bash
# Install toolchain extras (once)
cargo install just typos-cli cargo-audit cargo-deny cargo-vet

# Run the server
just run

# Health check
curl http://localhost:8080/health

# Create a user
curl -X POST http://localhost:8080/api/v1/users \
  -H 'Content-Type: application/json' \
  -d '{"email":"alice@example.com","display_name":"Alice"}'

# List users
curl http://localhost:8080/api/v1/users
```

## Recipes

| Command          | Description                               |
|------------------|-------------------------------------------|
| `just build`     | Debug build                               |
| `just run`       | Start server with `RUST_LOG=debug`        |
| `just test`      | Run all tests                             |
| `just fmt`       | Format code                               |
| `just clippy`    | Lint (deny warnings)                      |
| `just typos`     | Spell-check source                        |
| `just audit`     | Check for known CVEs                      |
| `just deny`      | License + advisory + ban check            |
| `just vet`       | Supply-chain verification                 |
| `just ci`        | Full local CI gate                        |

## Architecture

```
┌──────────────────────────────────┐
│           server crate           │  Actix Web, routes, handlers, state
│                                  │
│   ↓ depends on (use cases only)  │
├──────────────────────────────────┤
│     app-core :: application      │  Use cases, commands/queries, ports
│                                  │
│   ↓ operates on                  │
├──────────────────────────────────┤
│       app-core :: domain         │  Entities, value objects, errors
│       (zero ext dependencies)    │
├──────────────────────────────────┤
│  app-core :: infrastructure      │  Concrete adapters (DB, cache, queue…)
└──────────────────────────────────┘
```

**Dependency rule:** dependencies flow inward only.
`server` → `application` → `domain`. `infrastructure` implements `application`
traits but never imports from `domain` directly beyond entities.

## Configuration

Set via environment variables with the `APP__` prefix:

| Variable          | Default       | Description              |
|-------------------|---------------|--------------------------|
| `APP__HOST`       | `0.0.0.0`     | Bind address             |
| `APP__PORT`       | `8080`        | Listen port              |
| `APP__LOG_LEVEL`  | `info`        | `RUST_LOG`-style filter  |
| `APP__ENV`        | `development` | `development/staging/production` |
