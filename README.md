# Ashscript

Monorepo for **Ashscript**, a programming game where players write code to command units on a hex grid.

This repository is a [Cargo workspace](https://doc.rust-lang.org/cargo/reference/workspaces.html)
consolidating what were previously separate repositories.

## Packages

| Package | Path | Kind | Description |
| --- | --- | --- | --- |
| [`ashscript-types`](packages/ashscript-types) | `packages/ashscript-types` | lib | Shared game/state types (components, intents, world, keyframes). The dependency hub used by the server and client. |
| [`mono-server`](packages/mono-server) | `packages/mono-server` | bin | Monolithic game server: runs the simulation engine — including intent processing (player scripts → intents → actions → state) — and serves world keyframes to clients over a WebSocket (`:3000/game-state`). |
| [`bevy-client`](packages/bevy-client) | `packages/bevy-client` | bin | The game client, built with [Bevy](https://bevyengine.org/). Renders the world streamed from the server. |

### Architecture

```
bevy-client  ──WebSocket (:3000/game-state)──►  mono-server  ──uses──►  ashscript-types
     │                                               │
     └────────────────── depends on ────────────────┴──────────────►  ashscript-types
```

Intent processing lives inside `mono-server`'s engine (`engine/bots.rs`,
`engine/actions/`): each tick it runs player scripts into intents, turns intents
into actions, and applies them to game state. (A former standalone
`intent-processor` RabbitMQ bridge experiment was superseded by this and removed;
its history remains in the archived `Ashscript-Game/intent-processor` repo.)

## Development

All commands run from the repository root.

```sh
# Type-check the whole workspace
cargo check

# Run the server
cargo run -p mono-server

# Run the client (dev features = dynamic linking + asset hot-reload, faster iteration)
cargo run -p bevy-client --features dev
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

## Workspace layout

Shared dependency versions are declared once in the root `Cargo.toml` under
`[workspace.dependencies]` and referenced from each package with
`{ workspace = true }`, keeping versions in lockstep. Build profiles (including
Bevy's recommended "optimize dependencies in dev" setup) also live at the root.

## License

Workspace crates are dual-licensed under `MIT OR Apache-2.0`, **except**
`bevy-client`, which is proprietary (it ships to players).
