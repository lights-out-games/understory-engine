# Notes for coding agents

Read `docs/architecture-digest.md` first, then `CONTRIBUTING.md`.

- The five core rules in `CONTRIBUTING.md` are hard constraints: integers only, no input or output, every change reported, no product concepts, golden hash discipline.
- Process order in `Engine::step` is part of the contract. Do not reorder without a breaking-change note.
- New pack fields need `#[serde(default)]` so existing packs still load, an entry in `docs/pack-format.md` and in `spec/pack.schema.json`, and a test.
- New `World` fields change every state hash. Batch them and note the break.
- Checks: `cargo fmt --all`, `cargo clippy --all-targets`, `cargo test`, `./scripts/build-wasm.sh && node scripts/check-wasm-hash.cjs`.
- Downstream: Heartlight Heroes vendors `dist/nodejs`. After an engine change it runs `npm run update-engine` and `npm run bot`.
