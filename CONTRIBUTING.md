# Contributing

Thank you for helping. This project is early; open an issue before a large change.

## Terms

- Contributions are licensed under MIT OR Apache-2.0, the same as the project ("inbound equals outbound").
- Sign off each commit with `git commit -s`. The sign-off certifies the [Developer Certificate of Origin 1.1](https://developercertificate.org/): you wrote the change or have the right to submit it.
- There is no contributor licence agreement and no copyright assignment.

## Rules for the core

1. **Determinism.** Integer arithmetic only in `crates/core`. No floating point, no clocks, no host randomness, no iteration over hash maps. Use `BTreeMap`, `BTreeSet` or indexed vectors.
2. **No input or output** in `crates/core`: no files, network or environment.
3. **Every change is reported.** If a process changes a value, it must emit a `Change` with the old and new value. The test `every_change_is_reported` enforces this for patches.
4. **No product concepts.** Nothing specific to one game or app belongs in the core. Express it as data in a pack, or keep it in the host.
5. **The golden hash.** If your change alters `golden_hash`, say so in the pull request and in `CHANGELOG.md` under "Breaking". Then update the pinned value and run `node scripts/check-wasm-hash.cjs`.

## Before you open a pull request

```sh
cargo fmt --all
cargo clippy --all-targets
cargo test
./scripts/build-wasm.sh && node scripts/check-wasm-hash.cjs
```
