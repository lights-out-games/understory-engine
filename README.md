# Understory (working name)

A small, deterministic, headless ecosystem simulation engine. It takes a content pack, a starting world, a seed and a stream of host commands, and returns state, events and snapshots. It draws nothing, stores nothing and talks to no network.

It is written in Rust and built to be embedded: as WebAssembly in browsers, mobile web views and Node, and as a native crate in servers. The same inputs give the same state hash on every platform.

> **Status: 0.1, early.** A working first slice sized for a paper-prototype game. The name is a placeholder that has not been through a trademark or package-name search. APIs and the state format will change.

## What 0.1 contains

| Area | In 0.1 |
|---|---|
| World | A grid of patches with integer soil, water and plant (biomass) stocks |
| Life | Agents with needs and routine slots; species that arrive and leave with habitat |
| Processes | Weather, activities, structures, growth and wilting, species; fixed order |
| Interface | `apply(commands)`, `step()`, `state()`, `hash()`, `snapshot()`, `restore()` |
| Determinism | Integer arithmetic only, built-in PCG32 generator, ordered collections; a golden hash checked natively and in WebAssembly |
| Bindings | WebAssembly for Node and browsers (JSON strings in and out); the Rust crate; a command-line runner |
| Not yet | C interface, native mobile bindings, populations as counts, typed-array queries, binary snapshots, data adapters, parallel regions |

## Try it

```sh
cargo test                                   # 10 tests, including the golden hash
cargo run -p understory-cli -- run \
  --pack packs/reference/pack.json --world packs/reference/world.json \
  --seed 42 --ticks 5 --events               # prints every event, then the hash

./scripts/build-wasm.sh                      # builds dist/nodejs and dist/web
node scripts/check-wasm-hash.cjs             # WebAssembly hash must equal the native golden hash
```

The WebAssembly build needs `rustup target add wasm32-unknown-unknown` and `cargo install wasm-bindgen-cli --version 0.2.100 --locked`.

## Use it from JavaScript

```js
const { Understory } = require('./dist/nodejs/understory_wasm.js');
const engine = new Understory(packJson, worldJson, 42);          // strings, and a 32-bit seed
engine.apply(JSON.stringify([{ type: 'set_routine', agent: 'ana', slot: 'home', activity: 'balcony_garden' }]));
const events = JSON.parse(engine.step());                         // everything that happened, with before and after values
const world = JSON.parse(engine.state());
console.log(engine.hash());                                       // 16 hex digits
```

## Use it from Rust

```rust
use understory_core::*;
let mut engine = Engine::new(pack, world_def, 42)?;
engine.apply(&[Command::SetRain { amount: 2 }]);
let events = engine.step();
```

## How a tick runs

1. **Weather.** Every patch loses `evaporation` water, then gains the tick's rain up to its capacity.
2. **Needs reset** to each need's baseline, plus any one-tick bonus the host queued.
3. **Activities.** Agents in order, slots in pack order; each activity applies its effects.
4. **Structures.** Patches in index order; each structure applies its effects.
5. **Growth.** Watered patches grow toward their soil level; dry patches with tall plants wilt.
6. **Species.** Arrive when their habitat exists; leave after `grace` ticks without it.

Every value that changes is reported in an event with its old and new value. A test replays the reported changes and checks they reproduce the new state.

## Layout

| Path | What | Licence |
|---|---|---|
| `crates/core` | The engine: `model.rs`, `engine.rs`, `rng.rs` | MIT OR Apache-2.0 |
| `crates/wasm` | WebAssembly binding | MIT OR Apache-2.0 |
| `crates/cli` | `understory run` and `understory validate` | MIT OR Apache-2.0 |
| `packs/reference` | A tiny example pack and world | CC BY 4.0 |
| `spec/` | Pack and world JSON Schemas; notes on the planned data profile | CC0 |
| `docs/` | Design, pack format, model description, architecture digest, research | CC BY 4.0 |

## Documents

- [Design](docs/design.md): licensing, language, architecture, scaling, data standards, governance
- [Architecture digest](docs/architecture-digest.md): what the code is today and the rules for changing it
- [Pack format](docs/pack-format.md): how to describe an ecosystem as data
- [Model description (ODD)](docs/ODD.md): the model in the format ecologists expect
- [Visual overview](docs/overview.html): diagrams (open in a browser)
- [Integrations](docs/integrations/): Biotope digest
- [Research](docs/research/): licensing and legal, languages and tooling, scale and data standards

## Limits to know before you build on it

- It is a model for play and learning. Its outputs are plausible, not validated, and must not be presented as predictions.
- Worlds are small: tens of tiles and agents. Nothing has been benchmarked.
- A change to any state hash is a breaking change. Saves and replays are only valid for the version that made them.

## Licence

Code: MIT OR Apache-2.0, at your option ([LICENSE-MIT](LICENSE-MIT), [LICENSE-APACHE](LICENSE-APACHE)). Reference content: CC BY 4.0. Schemas: CC0. Contributions are accepted under the same terms with a Developer Certificate of Origin sign-off; see [CONTRIBUTING.md](CONTRIBUTING.md). The name and logo are not covered by the code licence; see [TRADEMARKS.md](TRADEMARKS.md).
