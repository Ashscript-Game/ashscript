# Ashscript

Monorepo for **Ashscript**, a programming game where players write code to command units on a hex grid.

This repository is a [Cargo workspace](https://doc.rust-lang.org/cargo/reference/workspaces.html)
consolidating what were previously four separate repositories.

## Packages

| Package | Path | Kind | Description |
| --- | --- | --- | --- |
| [`ashscript-types`](packages/ashscript-types) | `packages/ashscript-types` | lib | Shared game/state types (components, intents, world, keyframes). The dependency hub used by the server and client. |
| [`mono-server`](packages/mono-server) | `packages/mono-server` | bin | Monolithic game server: runs the simulation engine and serves world keyframes to clients over a WebSocket (`:3000/game-state`). |
| [`bevy-client`](packages/bevy-client) | `packages/bevy-client` | bin | The game client, built with [Bevy](https://bevyengine.org/). Renders the world streamed from the server. |
| [`intent-processor`](packages/intent-processor) | `packages/intent-processor` | bin | ⚠️ **Experimental / not currently wired in.** A standalone RabbitMQ ⇄ Socket.IO bridge node. Kept for reference; not part of the live server/client loop. |

### Architecture

```
bevy-client  ──WebSocket (:3000/game-state)──►  mono-server  ──uses──►  ashscript-types
     │                                               │
     └────────────────── depends on ────────────────┴──────────────►  ashscript-types

intent-processor  (experimental, standalone — RabbitMQ/Socket.IO; not in the live loop)
```

## Development

All commands run from the repository root.

```sh
# Type-check the whole workspace
cargo check

# Run the server
cargo run -p mono-server

# Run the client (dev features = dynamic linking + asset hot-reload, faster iteration)
cargo run -p bevy-client --features dev

# Run the experimental intent-processor (needs a reachable RabbitMQ broker)
cargo run -p intent-processor
```

### Shipping the client

```sh
cargo build -p bevy-client --profile distribution \
  -F tracing/release_max_level_error -F log/release_max_level_off
```

## Configuration

Runtime secrets/config are read from environment variables (and a gitignored
`.env` at the repo root). Copy [`.env.example`](.env.example) to `.env` and fill in
real values:

| Variable | Used by | Default | Purpose |
| --- | --- | --- | --- |
| `MONO_SERVER_BIND` | mono-server | `0.0.0.0:3000` | game-state WebSocket bind address |
| `RABBITMQ_URL` | intent-processor | `amqp://guest:guest@localhost:5672/%2f` | RabbitMQ connection string |
| `INTENT_PROCESSOR_BIND` | intent-processor | `0.0.0.0:3000` | HTTP/Socket.IO bind address |

## Workspace layout

Shared dependency versions are declared once in the root `Cargo.toml` under
`[workspace.dependencies]` and referenced from each package with
`{ workspace = true }`, keeping versions in lockstep. Build profiles (including
Bevy's recommended "optimize dependencies in dev" setup) also live at the root.

## License

Workspace crates are dual-licensed under `MIT OR Apache-2.0`, **except**
`bevy-client`, which is proprietary (it ships to players).
