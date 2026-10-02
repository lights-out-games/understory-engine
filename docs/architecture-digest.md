# Architectural Digest: Engine Repo

Working name: **Understory** (placeholder; run a trademark and package-name search before any public use).
Status: version 0.1, October 2, 2026. The first slice is built and tested; nothing is benchmarked.
Read with: [design.md](design.md) for the reasoning and the longer plan.

## 1. What this repo is

A deterministic, headless ecosystem simulation library. It takes a content pack, a starting world, a seed and host commands, and returns state, events and snapshots. It has no rendering, storage, networking or user data.

## 2. Dependency direction

```
heartlight-heroes  --+
biotope            --+-->  understory (this repo)  -->  permissive third-party crates only
third-party hosts  --+
```

- This repo never depends on, imports from, or names a host product.
- Hosts pin an exact engine build. Replays and saves are only valid for the version that made them.
- Anything specific to one product (Heartlight, Darkroots, missions, scoring) stays in that product's repo.

## 3. Layout

| Path | Purpose | Built in 0.1 |
|---|---|---|
| `crates/core` | World, agents, processes, random generator, snapshot, state hash | Yes |
| `crates/wasm` | wasm-bindgen binding, JSON strings in and out | Yes |
| `crates/cli` | `run`, `validate` | Yes |
| `packs/reference` | Tiny example pack and world | Yes |
| `spec/` | JSON Schemas for pack and world; data profile notes | Schemas yes; profile is notes only |
| `crates/content`, `crates/data`, `crates/ffi-c`, `packages/web` | Pack tooling, data adapters, C interface, npm wrapper | Not yet |

## 4. Public interface (the only thing hosts may rely on)

| Call | Contract |
|---|---|
| `Engine::new(pack, world, seed)` | Validates the pack and world; fails with a message that names the problem |
| `apply(commands)` | Applied immediately, in order, between ticks. Returns events and a list of rejected commands with reasons. A rejected command changes nothing |
| `step()` | Advances one tick. Returns every event, each with the values it changed |
| `state_json()` / `world()` | Full state, ordered, read-only |
| `hash()` | FNV-1a 64 of the state JSON, 16 hex digits. Identical on every platform for identical inputs |
| `snapshot()` / `restore(pack, snapshot)` | JSON. Restore refuses a different pack or a different engine minor version |

Commands: `set_routine`, `place_structure`, `remove_structure`, `adjust_patch`, `adjust_need` (one-tick bonus), `set_rain` (next tick only).

Events carry `phase` (`commands`, `weather`, `activities`, `structures`, `growth`, `needs`, `species`), `kind`, optional `subject`, `detail`, `value`, and `changes`.

## 5. Where 0.1 differs from the design document

| Design said | 0.1 does | Why |
|---|---|---|
| Commands queue until the next tick | Commands apply immediately between ticks | Hosts show the result of a planning action at once; the host's command log still records the tick |
| Restricted floating-point math | No floating point at all | A paper-scale model needs none, and it removes the hardest determinism risk |
| Binary snapshots (postcard) | JSON snapshots | Viewable and diffable while the format is still moving |
| Typed-array queries | Whole state as JSON | Worlds are tiny; revisit when a host needs speed |
| Populations as counts per patch | Species are present or absent | Enough for the first game; counts are the next modelling step |
| `u64` seed | `u32` seed, and generator state stored as two `u32` halves | JavaScript hosts cannot round-trip 64-bit integers through JSON |

## 6. Determinism rules (tests and review enforce these)

1. One tick is one call to `step`. No wall-clock time, no host randomness.
2. Integer arithmetic only in `crates/core`.
3. The generator is the PCG32 in `rng.rs`. Do not swap it for a crate.
4. Ordered collections only (`BTreeMap`, `BTreeSet`, vectors).
5. Processes run in the order written in `Engine::step`.
6. Golden test: the reference scenario must hash to the pinned value natively and in WebAssembly (`scripts/check-wasm-hash.cjs`).

## 7. Extension points

| Level | Mechanism | Available |
|---|---|---|
| Content | JSON content packs: terrains, weather, needs, activities, structures, species | Yes |
| Host rules | Commands and events between ticks | Yes |
| Compiled process | A `Process` trait | Not yet; processes are inline in `step` |
| Runtime plugin | Sandboxed WebAssembly process | Deferred; server only |

## 8. Build and release

- Rust stable, Cargo workspace, minimum Rust 1.81.
- `./scripts/build-wasm.sh` builds `dist/nodejs` and `dist/web` with wasm-bindgen 0.2.100 (the CLI version must match the crate version).
- Dependency licences: permissive only (`deny.toml`); the current list is in `THIRD-PARTY.md`.
- Not yet set up: crates.io and npm publishing, release automation, REUSE headers.

## 9. Do and do not

| Do | Do not |
|---|---|
| Add behaviour as data first | Add product concepts to the core |
| Report every changed value in an event | Mutate state without a `Change` |
| Add `#[serde(default)]` to new pack fields | Break existing packs silently |
| Note every golden-hash change as breaking | Read files, clocks or the network from `crates/core` |
