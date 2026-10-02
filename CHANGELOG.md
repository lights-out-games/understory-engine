# Changelog

## 0.1.0 (unreleased)

First slice.

- Core: patches with soil, water and biomass; agents with needs and routine slots; activities, structures, growth and wilting, species with a grace period; weather with a seeded die roll.
- Interface: `apply`, `step`, `state_json`, `hash`, `snapshot`, `restore`.
- Bindings: WebAssembly (Node and web targets), command-line runner.
- Determinism: golden hash `da24e4417bf845b2` for the reference scenario, identical natively (x86-64 Linux) and in WebAssembly under Node 22. Other platforms and browsers are not yet in CI.
