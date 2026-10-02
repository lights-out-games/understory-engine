# Research report: embeddable deterministic ecosystem-sim engine (as of 2026-10-01)

Method notes. Every bullet cites a page I opened, or a registry API I queried (crates.io / npm JSON, marked "registry"). Items I could not confirm from an opened source are marked **not verified**. Page summaries came through a fetch-and-summarise tool, so anything quoted from prose pages deserves a spot check before it is relied on; registry versions and dates are exact.

## 1. Prior art

### Scientific / ABM frameworks
| Project | Language | Licence | Embeddability / notes | Status | Source |
|---|---|---|---|---|---|
| NetLogo | Scala/Java (JVM) | GPL-2.0 | Desktop app; JVM only | v7.0.4, 2026-05-01, active | https://github.com/NetLogo/NetLogo |
| Tortoise (NetLogo Web engine) | Scala / Scala.js, some CoffeeScript | Reported as EPL-1.0, inferred by the summariser; **not verified** | Compiles NetLogo models to JS; ships `tortoise-compiler.js` and `tortoise-engine.js` | 2,286 commits, maintained | https://github.com/NetLogo/Tortoise |
| Mesa | Python | Apache-2.0 | Python only | Mesa 3 stable, Mesa 4 pre-releases, GSoC 2026 | https://github.com/projectmesa/mesa |
| Agents.jl | Julia | MIT | Julia only | v7.0.3, 2026-06-23 | https://github.com/JuliaDynamics/Agents.jl |
| Repast4Py (plus Repast HPC C++) | Python + C++ (MPI) | BSD-3 | HPC-oriented | 9 tagged releases, active | https://github.com/Repast/repast4py |
| MASON | Java | Licence file present, type **not verified** | JVM | 1,673 commits; v22.0 used in benchmarks | https://github.com/eclab/mason |
| GAMA | Java | GPL-3.0 | Has a headless product; JVM/Eclipse-based | Active, 2,685 commits | https://github.com/gama-platform/gama |
| krABMaga | Rust | MIT | Rust crate; wasm via `visualization_wasm` feature; visualisation through Bevy; MPI exploration. Re-engineered from MASON's architecture | v0.6.2, 2026-06-19 | https://github.com/krABMaga/krABMaga, crates.io registry |
| FLAME GPU 2 | C++/CUDA + Python | AGPL-3.0-only or commercial | Requires an NVIDIA GPU (CUDA 12+), C++20 | Active | https://github.com/FLAMEGPU/FLAMEGPU2 |
| AgentScript | ES6 JS modules | GPL-3.0 | Model layer separate from views (Canvas 2D, Three.js, Leaflet/MapLibre); NetLogo semantics | npm 0.10.27, 2025-12-12 | https://github.com/backspaces/agentscript, npm registry |
| Madingley | C# (UNEP-WCMC core version); MadingleyR (R + C++) exists per search results only | "Free… open to use and modify"; specific licence **not verified** | Research code needing external environmental datasets | 23 commits; activity **not verified** | https://github.com/Madingley/C-sharp-version-of-Madingley, https://madingley.github.io/ |
| LANDIS-II Core v8 | C# / .NET | Licence file present, type **not verified** | Installer plus extensions | 36 commits | https://github.com/LANDIS-II-Foundation/Core-Model-v8 |
| iLand | C++ / Qt | GPL-3.0 | Individual-tree forest landscape model, 10,000+ ha; used in 40+ papers | 1,121 commits, active | https://github.com/edfm-tum/iland-model |
| LPJ-GUESS | C++ (**not verified** on the page) | MPL-2.0 | Dynamic vegetation model, local to global | v4.1.1 public release, Oct 2021 | https://zenodo.org/records/8065737 |
| Ecopath with Ecosim | .NET Framework 4.8, Windows; needs MS Access drivers | Free and Pro editions; licence **not verified** | Windows-bound | 6.7 beta, May 2024 | https://ecopath.org/downloads/ |
| Ecolab, EcoSim (Gras), "Biomes" | — | — | — | **not verified** (not researched) | — |

### Game-side
- **Eco (Strange Loop Games):** the server runs a 24/7 simulation in which "thousands of plants and animals" are individually simulated rather than statistically modelled. One real day is roughly one simulated year. The server tracks population, hunting, pollution, birth and death rates, which players can graph in-game. Source: https://www.moddb.com/games/eco-global-survival-game/news/how-the-eco-sim-works-the-players-lifeline (date not captured). The C# server and `EcoSim.eco` config are **not verified**.
- **Rain World:** offscreen creatures run in an "abstract" form and are "realized" near the player. This is **not verified**: the wiki page returned 403 and the 80.lv article had no technical detail. Wiki URL for follow-up: https://rainworld.miraheze.org/wiki/Technical_Glossary/Abstractization
- **Sapiens (Majic Jungle):** mods are Lua scripts that mirror and override vanilla GameResources via load order and `onLoad` hooks. Source: https://github.com/Majic-Jungle/sapiens-mod-creation/wiki/. The C/LuaJIT engine split is **not verified**.
- **Equilinox:** closed-source Java game; only community mod frameworks appeared in search results (not opened). **Not verified.**
- **SimEarth / SimLife, Dwarf Fortress:** **not verified** (not researched).

### ECS foundations
| Library | Language | Licence | Version / date | Notes | Source |
|---|---|---|---|---|---|
| bevy_ecs | Rust | MIT OR Apache-2.0 | 0.19.1, 2026-08-13 (0.20.0-rc.2, 2026-09-28) | MSRV 1.95, a very recent toolchain. Order of non-conflicting systems is nondeterministic by default; needs `.before/.after/.chain` | crates.io registry; https://bevy-cheatbook.github.io/programming/system-order.html (which Bevy version the cheatbook covers is **not verified**) |
| hecs | Rust | MIT OR Apache-2.0 | 0.11.1, 2026-07-28 | Minimal archetype ECS, library not framework | https://github.com/Ralith/hecs, registry |
| flecs | C99 / C++17 | MIT | Version not captured | Archetype/SoA, relationships, JSON reflection, runs in wasm via Emscripten, 13,000+ tests. Rust binding `flecs_ecs` 0.2.2 (2025-11-17) | https://github.com/SanderMertens/flecs, registry |
| EnTT | C++20 header-only | MIT | 3.16.0 | Sparse-set; used by Minecraft | https://github.com/skypjack/entt |
| bitECS | TypeScript | MPL-2.0 | 0.4.0, 2025-12-06 | SoA/AoS, about 5 kB min+gzip, serialisation built in | https://github.com/NateTheGreatt/bitECS, npm |
| Koota (pmndrs) | TypeScript | ISC | 0.6.6, 2026-04-09 | SoA traits; React hooks (`useQuery`, `useTrait`) | https://github.com/pmndrs/koota, npm |

### Gap statement
- I found no existing library that is at once (a) an ecosystem model with patch stocks/flows plus populations plus agents, (b) deterministic and replayable, (c) headless and embeddable, and (d) shipped for web, mobile and native.
- The nearest pieces each miss something. krABMaga is a general Rust ABM with wasm visualisation, but makes no cross-platform determinism claim and has no ecology domain model. AgentScript is JS-only and GPL. Tortoise is JS-only with NetLogo semantics. The scientific models are tied to the JVM, .NET, Qt or CUDA.
- One search for a Rust/wasm ecosystem-sim crate returned only hobby repos (bones-ai/rust-ecosystem-simulation, willfarris/ecosimulation, czlin7/EcoSimEngine), none opened. Absence is from search, not proof.

## 2. Core language options

### Rust
- **Toolchain:** stable is 1.98.1 (2026-09-03), a point release fixing a vtable miscompilation: https://blog.rust-lang.org/2026/09/03/Rust-1.98.1/. Edition 2024 shipped in 1.85.0 (2025-02-20): https://doc.rust-lang.org/edition-guide/rust-2024/index.html
- **Target tiers:** all Tier 2: wasm32-unknown-unknown, wasm32-wasip1/p2/p3, wasm32-unknown-emscripten, wasm32v1-none, aarch64-apple-ios (and -sim), and the four Android targets. wasm64-unknown-unknown is Tier 3. Source: https://doc.rust-lang.org/nightly/rustc/platform-support.html (nightly docs).
- **rustwasm archival:** the Rust project announced on 2025-07-21 that the rustwasm GitHub org would be archived in September 2025; wasm-bindgen moved to a new `wasm-bindgen` org with additional maintainers. Source: https://blog.rust-lang.org/inside-rust/2025/07/21/sunsetting-the-rustwasm-github-org. The working group had been inactive since July 2024; walrus and weedle also moved to the new org; gloo went to ranile and twiggy to AlexEne: https://zenn.dev/lemonadern/articles/bb8fd4715dd33f?locale=en (2025-08-10).
- **wasm-bindgen today:** 0.2.129, released 2026-09-25, MSRV 1.81 under a "2-year policy", with a new Emscripten build flow and experimental JSPI: https://github.com/wasm-bindgen/wasm-bindgen/releases
- **wasm-pack today:** it first went to drager, and that repo now says it "has moved to wasm-bindgen/wasm-pack": https://github.com/drager/wasm-pack. crates.io shows wasm-pack 0.15.0 on 2026-05-15, so it is still released.
- **Alternative to wasm-pack:** `cargo build`, then `wasm-bindgen --target web`, then `wasm-opt -O`. Stated pros: per-platform build logic, a pinned wasm-opt, parallel bundles. Stated cons: manual tool management and losing package.json generation and publish helpers. Source: https://nickb.dev/blog/life-after-wasm-pack-an-opinionated-deconstruction/ (2025-09-18).
- **UniFFI:** 0.32.2 (2026-09-23), MPL-2.0, still 0.x. Kotlin, Swift and Python have full support; Ruby is kept working only. Interfaces are defined by proc-macros or UDL. Third-party bindings "may require older versions". The docs say it "will not help you ship a Rust library to these platforms", so packaging is on you. Source: https://mozilla.github.io/uniffi-rs/latest/, registry.
- **uniffi-bindgen-react-native:** 0.31.0-6 (2026-09-25), MPL-2.0. Generates TypeScript for React Native and also for web (wasm) and Node from the same UniFFI annotations: https://github.com/jhugman/uniffi-bindgen-react-native
- **C ABI:** cbindgen 0.29.4 (2026-06-09), MPL-2.0 (registry).
- **Unity:** csbindgen 1.9.8 (2026-05-20), MIT. Generates C# `DllImport` from Rust `extern "C"`; has a `MonoPInvokeCallback` mode because Unity lacks function pointers, and `__Internal` naming for iOS: https://github.com/Cysharp/csbindgen
- **Node:** @napi-rs/cli 3.10.6 (2026-10-01), MIT; the napi 3.x line has wasm-target support: npm registry, https://github.com/napi-rs/napi-rs/releases (crate release date **not verified**).
- **Godot:** `godot` crate (gdext) 0.5.5 (2026-08-09), MPL-2.0, Godot 4 only. Desktop is primary; wasm, Android and iOS are "experimental"; occasional breaking changes: https://github.com/godot-rust/gdext
- **Flutter:** flutter_rust_bridge 2.13.0 (2026-08-23), MIT, a Flutter Favorite; covers Android, iOS, desktop and web: https://github.com/fzyzcjy/flutter_rust_bridge
- **Binary size of a small Rust wasm library:** **not verified**, no figures gathered.

### Other languages
- **C/C++ (Emscripten):** flecs and EnTT show the pattern; mlua's wasm support is specifically via `wasm32-unknown-emscripten`: https://github.com/mlua-rs/mlua. Emscripten version and size figures **not verified**.
- **Zig:** 0.16.0 (2026-04-13), still pre-1.0: https://ziglang.org/download/
- **AssemblyScript:** 0.28.20 (2026-07-22). No closures, union types, exceptions or async. Threads, GC and exception handling are "in development". It marks the Component Model and WASI as "harmful". Wasm-only, so no native target. Source: https://www.assemblyscript.org/status.html
- **TypeScript-only:** runs wherever JS runs. bitECS and Koota show SoA typed-array designs. Native engine hosts would need to embed a JS engine. No source opened on TS-only determinism or performance.
- **Kotlin Multiplatform:** stable since 1.9.20; Kotlin/Native stable; Kotlin/Wasm is Beta (since 2.2.20); C/Objective-C interop is Beta: https://kotlinlang.org/docs/components-stability.html. Binary size **not verified**.
- **Go / TinyGo:** TinyGo builds wasm with `GOOS=js GOARCH=wasm` and needs its own `wasm_exec.js` glue: https://tinygo.org/docs/guides/webassembly/wasm/. Sizes **not verified**.
- **C#:** Native AOT (.NET 8+) is supported on Windows, Linux and macOS; iOS and Android are "Experimental" (Android has no Java interop). No dynamic loading or Reflection.Emit: https://learn.microsoft.com/en-us/dotnet/core/deploying/native-aot/. A wasm path and C-export details are **not verified**. Photon Quantum is the precedent for a deterministic C# simulation (section 4).

## 3. WebAssembly platform status

- **Wasm 3.0** was announced 2025-09-17. It adds memory64, multiple memories, GC, typed references, tail calls, exception handling, relaxed SIMD, a deterministic profile, custom annotations and JS string builtins. Source: https://webassembly.org/news/2025-09-17-wasm-3.0/
- **Feature support** (first supporting version, from https://raw.githubusercontent.com/WebAssembly/website/main/features.json, fetched 2026-10-01):

| Feature | Chrome | Firefox | Safari | Node | Wasmtime |
|---|---|---|---|---|---|
| Fixed SIMD | 91 | 89 | 16.4 | 16.4 | 0.33 |
| Threads | 74 | 79 | 14.1 (iOS 14.5) | 16.4 | 15 |
| WasmGC | 119 | 120 | 18.2 | 22 | 47 |
| memory64 | 133 | 134 | flag only (TP 251) | 24 | 30 |
| Relaxed SIMD | 114 | 145 | flag only | 21 | 15 |
| Exceptions (final) | 137 | 131 | 18.4 | 24.15 | 47 |
| Multi-memory | 120 | 125 | not listed | 22 | 15 |
| JSPI | 137 | 153 | 27 | 26 | n/a |

- **Deterministic profile:** in that matrix only Wasmtime is flagged as implementing it; browsers show null.
- **Threads / SharedArrayBuffer:** needs a secure context plus cross-origin isolation (COOP `same-origin` with COEP `require-corp` or `credentialless`): https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/SharedArrayBuffer
- **WebViews:** caniwebview lists SharedArrayBuffer/Atomics as not supported in Android WebView, WKWebView and WebView2: https://caniwebview.com/features/web-feature-shared-memory/. That conflicts in part with the Capacitor issue below, so treat WebView SAB as unreliable. The current Chromium status (issue 40914606) was unreadable; **not verified**.
- **Capacitor specifically:** issue #6182 (closed) says that on iOS the WKWebView URL-scheme handler does not honour COOP/COEP on initial load, so there is no SAB. On Android, WebView "seems to not support COOP header… even if it supports SharedArrayBuffer". The workaround named is a local HTTP server instead of the scheme handler. Source: https://github.com/ionic-team/capacitor/issues/6182
- **Lockdown Mode:** Apple says it blocks "certain complex web technologies" and that individual apps and sites can be excluded in Settings; the page does not name WebAssembly: https://support.apple.com/en-us/105120. That it disables wasm/JIT in WKWebView is supported only by search-result titles (e.g. flutter/flutter#114549); **not verified**.
- **WKWebView JIT for wasm inside third-party apps:** **not verified**.
- **Performance:** "Not So Fast" (Jangda et al., 2019, SPEC CPU) found wasm 1.55x slower than native on average in Chrome (peak 2.5x) and 1.45x in Firefox (peak 2.08x). Causes given: more loads and stores, more branches from safety checks, larger code. Source: https://ar5iv.labs.arxiv.org/html/1901.09056. This predates SIMD and newer JIT tiers. Recent wasm-vs-JS figures are **not verified**.
- **WASI / Component Model:** WASI 0.3.0 launched 2026-06-11 with native async (`stream<T>`, `future<T>`). Wasmtime 45 ran the RC and 46 ships it on by default. Jco supports it. Guest toolchains are "in active development". Component Model 1.0 is still ahead. Source: https://bytecodealliance.org/articles/WASI-0.3. Wasmtime is at 49.0.1 (2026-09-24, MSRV 1.96; registry).
- **Wasmtime on mobile:** iOS and Android targets are Tier 3 (no CI, no full-time maintainer). The Pulley interpreter is Tier 2 and is the portable no-JIT path, without threads: https://docs.wasmtime.dev/stability-tiers.html
- **Streaming compile:** `WebAssembly.instantiateStreaming` requires the `application/wasm` content type (stated on the TinyGo page above).
- **Not verified:** worker `postMessage` transfer semantics; Android WebView minimum versions per wasm feature (Capacitor 8 itself requires Android 7.0 / API 24 and iOS 15, see section 5).

## 4. Determinism

- **Wasm spec:** float operations are deterministic except NaN payloads. Canonical-NaN inputs give a canonical output; otherwise the payload is "picked non-deterministically among all arithmetic NaNs". The deterministic profile yields a positive canonical NaN. Source: https://webassembly.github.io/spec/core/exec/numerics.html
- **Relaxed SIMD:** results "may vary based on hardware support" for relaxed_swizzle, relaxed_trunc, relaxed_madd/nmadd (single versus double rounding), relaxed_laneselect, relaxed_min/max, relaxed_q15mulr and relaxed_dot. Each environment fixes its choice, so it is consistent within one environment but not across them. Source: https://github.com/WebAssembly/relaxed-simd/blob/main/proposals/relaxed-simd/Overview.md
- **Wasmtime guidance:** deterministic imports only; `cranelift_nan_canonicalization` (costs float performance); disable relaxed SIMD or set `relaxed_simd_deterministic`; bound or pre-allocate memory growth; use fuel rather than epoch interruption. Source: https://docs.wasmtime.dev/examples-deterministic-wasm-execution.html
- **Rust std math:** sin, cos, tan, the inverse and hyperbolic functions, exp, ln, log*, powi, powf, cbrt and hypot are documented as non-deterministic precision: "varies by platform, Rust version, and can even differ within the same execution". `sqrt`, `mul_add`, floor, ceil, round, trunc and `rem_euclid` are exact and guaranteed. Source: https://doc.rust-lang.org/std/primitive.f64.html
- **libm crate:** 0.2.16, MIT, pure-Rust math functions: https://docs.rs/libm/latest/libm/. Its docs page makes no explicit cross-platform bit-identity promise, so "libm as the fix" is common practice, **not verified** as a documented guarantee.
- **Fixed point:** `fixed` 1.31.0 (2026-03-20; registry). Photon Quantum uses fixed-point math plus a deterministic ECS with predict/rollback, exchanging only inputs: https://doc.photonengine.com/quantum/current/quantum-intro
- **rand policy:** `StdRng` and `SmallRng` are explicitly non-portable and may change in any release. Named PRNG crates are expected to be reproducible across platforms and patch releases, with value-breaking changes allowed in minor versions and noted in the changelog. "Never sample a usize or isize" if you need portability. Float results from `rand_distr` "may not be portable". Source: https://rust-random.github.io/book/crate-reprod.html
- **PRNG versions (registry):** rand 0.10.3 (2026-09-20), rand_chacha 0.10.0, rand_pcg 0.10.2, rand_xoshiro 0.8.1.
- **Rapier** (rapier3d 0.36.0, 2026-09-25): locally deterministic by default (same machine, Rapier version and rustc). Cross-platform determinism requires the `enhanced-determinism` feature, IEEE 754-2008 compliant targets, and using nalgebra's `ComplexField`/`RealField` math rather than std `.sin()`. It is incompatible with `simd8` as summarised (verify the exact feature names) and compatible with `parallel`. Source: https://rapier.rs/docs/user_guides/rust/determinism/
- **Box2D v3:** cross-platform determinism (x64/ARM; MSVC/GCC/Clang) without fixed point. Achieved with `-ffp-contract=off`, no fast-math, and custom sin/cos/atan2 ("atan2f gives different answers on different platforms"). Multithreading made it "much more challenging"; rollback is not supported. Source: https://box2d.org/posts/2024/08/determinism/
- **Factorio (FFF-52, Sept 2014):** float discrepancies "hasn't been a big problem"; they wrote custom trig functions. A real desync came from an ambiguous sort comparator, since `std::sort` orders equal elements differently across compilers. Source: https://www.factorio.com/blog/post/fff-52
- **Iteration order:** Bevy's default system order is nondeterministic (section 1). Rust `HashMap`'s per-process random seeding was not opened; **not verified here** (well known).

## 5. Mobile embedding

- **Capacitor:** v8 is current (released 2025-12-08); v7 maintenance ended 2026-06-08. v8 requires Node 22, Xcode 26, iOS 15+ and Android 7.0 (API 24)+: https://capacitorjs.com/docs/main/reference/support-policy. @capacitor/core is 8.5.2 (2026-09-11; npm). 8.5 (2026-07-31) adopted UIScene because iOS 27 SDK apps must; Capacitor 9 is planned: https://ionic.io/blog/capacitor-8-5-released
- **Capacitor plugins:** native side in Swift/Obj-C and Java/Kotlin, a web/PWA implementation is possible, and JS hooks are generated: https://capacitorjs.com/docs/plugins. Bridge serialisation format and cost are **not verified**.
- **Trade-off, wasm in the WebView versus a native plugin:** wasm in the WebView reuses the web build but has no threads (section 3). A native Rust plugin via UniFFI gets native threads but crosses the bridge. This is inferred from the sources above, not stated in any one of them.
- **Packaging:** the UniFFI XCFramework docs page returned 404; **not verified**. Apple distributes XCFrameworks as SwiftPM binary targets with URL plus checksum: https://developer.apple.com/documentation/xcode/distributing-binary-frameworks-as-swift-packages (page is JS-rendered; details thin). The `xcframework` crate is 0.2.1 (registry).
- **Android:** cargo-ndk builds the `jniLibs` layout for armeabi-v7a, arm64-v8a, x86_64 and x86; MSRV 1.86: https://github.com/bbqsrc/cargo-ndk. 16 KB page-size handling **not verified**.
- **React Native:** Nitro Modules 0.37.1 (C++/Swift/Kotlin over JSI). Its own benchmark for 100k calls: Nitro 7.27 ms, TurboModules 115.86 ms, Expo 434.85 ms: https://nitro.margelo.com/docs/what-is-nitro. uniffi-bindgen-react-native is in section 2.
- **Apple 2.5.2:** apps may not "download, install, or execute code which introduces or changes features or functionality of the app"; there is an educational-app exception requiring viewable, editable source. Guideline 4.7 allows HTML5/JS mini apps and mini games, plug-ins and emulators subject to 4.7.1–4.7.5 (privacy, content filtering, no exposing native APIs without permission, a software index, age restriction). Source: https://developer.apple.com/app-store/review/guidelines/
- **Data-only packs versus downloaded scripts or wasm:** the guideline does not address this directly; **not verified**.
- **App size impact:** **not verified**.

## 6. Desktop and native engine hosts

- **Tauri:** 2.12.1 (2026-09-30); 3.0.0-alpha.4 is on crates.io (2026-10-01). Tauri 2.0 went stable 2024-10-02. A Verso (Servo) integration was previewed 2025-03-17. Sources: registry, https://v2.tauri.app/blog/. Tauri 3 scope and mobile maturity **not verified**.
- **Electron:** 44.5.1 (2026-09-30; npm).
- **Steam:** the platform docs do not mention HTML5 or Electron. Requirements include 64-bit and notarised macOS builds, and Steam is incompatible with the macOS app-sandbox entitlement: https://partner.steamgames.com/doc/store/application/platforms. Shipping web builds on Steam is common but **not verified** here.
- **Unity:** native plugins are C-linkage libraries called via `[DllImport]`; iOS links statically and uses `__Internal`: https://docs.unity3d.com/6000.2/Documentation/Manual/plug-ins-native.html. WebGL and IL2CPP specifics **not verified**.
- **Godot:** GDExtension loads native shared libraries at runtime without recompiling the engine. An extension built for 4.2 should work in 4.3 but not the reverse; the page calls it experimental, and float precision must match the engine build. Source: https://docs.godotengine.org/en/stable/tutorials/scripting/gdextension/what_is_gdextension.html
- **Unreal:** an `External` module in `.build.cs` with `PublicAdditionalLibraries`, `RuntimeDependencies` and `PublicDelayLoadDLLs`; rpath rules differ per OS: https://dev.epicgames.com/documentation/en-us/unreal-engine/integrating-third-party-libraries-into-unreal-engine
- **C ABI as common denominator:** all three engines consume C-linkage libraries, and cbindgen and csbindgen target exactly that.

## 7. Server

- **Node:** two routes. The wasm build is one artifact with the same bits as the browser. A napi-rs addon is native speed with threads but needs per-platform binaries; napi-rs can also target wasm. No opened source benchmarks the two; **not verified**.
- **Cloudflare Workers:** 128 MB per isolate "including the JavaScript heap and WebAssembly allocations". CPU is 10 ms on the free plan and up to 5 minutes on paid. Bundles up to 64 MiB. Global-scope startup must finish within 1 s. Source: https://developers.cloudflare.com/workers/platform/limits. The fetch reported 64 MiB for both tiers and "5 minutes default"; check the page for the per-plan split.
- **Durable Objects:** 30 s CPU per request by default, configurable to 5 minutes. SQLite-backed objects hold 10 GB each with a 2 MB key+value limit. About 1,000 requests/s soft limit per object. WebSocket messages up to 32 MiB. Source: https://developers.cloudflare.com/durable-objects/platform/limits
- **Deno / Bun:** Deno's wasm feature support is in the matrix (GC 1.38, memory64 2.2). Bun specifics **not verified**.
- **Many sims per process, axum/tokio hosting:** **not verified** (no source opened).
- **bincode:** RUSTSEC-2025-0141 (published 2025-12-16) marks it unmaintained; development ceased after a doxxing and harassment incident. The advisory suggests wincode, postcard, bitcode or rkyv: https://osv.dev/vulnerability/RUSTSEC-2025-0141. crates.io shows a final bincode 3.0.0 on 2025-12-16.
- **postcard:** 1.1.3 (2025-07-24). "Documented and stable wire format" since 1.0; `no_std` focus; varint integers. Not self-describing, so schema evolution is manual (my inference; the docs page was silent). Source: https://docs.rs/postcard/latest/postcard/
- **rkyv:** 0.8.18 (2026-08-05). Zero-copy; bytecheck validation; rend for endian-agnostic archives: https://rkyv.org/. Format-stability and evolution policy **not verified**.
- **Others (registry only):** bitcode 0.6.9, wincode 0.6.2 (2026-09-28), rmp-serde 1.3.1, flatbuffers 25.12.19, serde 1.0.229. Schema-evolution comparisons **not verified**.

## 8. Scripting, modding, extension

- **Rhai:** 1.26.1 (2026-09-10), MSRV 1.66. Sandboxed ("cannot mutate the containing environment unless explicitly permitted"). Supports no-std and wasm. Has operation, stack and data-size limits. "1 million iterations in 0.14 sec". Compiles once to an AST. Source: https://rhai.rs/book/about/features.html
- **mlua:** 0.12.1 (2026-08-29), MIT. Lua 5.1–5.5, LuaJIT and Luau. `Lua::sandbox` for Luau. Wasm only via `wasm32-unknown-emscripten`, with no JIT there. Source: https://github.com/mlua-rs/mlua
- **Rune** 0.14.2 (2026-05-22) and **cel** 0.14.5 (2026-09-07): registry only; features **not verified**.
- **Wasmi:** 1.0 announced 2025-12-03; now 2.0.0 (2026-09-01). An interpreter for embedded and plugin use. Supports Wasm 2.0 plus memory64, multi-memory, opt-in SIMD and relaxed SIMD. No GC, exception handling or function references. Fuel metering with "refueled resumable calls". Only two dependencies. Source: https://wasmi-labs.github.io/blog/posts/wasmi-v1.0/. Its use on iOS-native where JIT is banned is the implied fit, not stated on the page.
- **Extism:** runtime crate 1.30.0 (2026-06-04), BSD-3. Host SDKs in 15+ languages. The JS SDK runs on Web, Node, Deno and Bun, but is still 2.0.0-rc13 (2025-05-14). Source: https://github.com/extism/extism, registries. Underlying engine **not verified**.
- **Determinism implications:** for wasm plugins see the Wasmtime checklist in section 4 (fuel, NaN canonicalisation, no relaxed SIMD). For Lua and Rhai, script-side float math and table iteration order are hazards; **not verified** from a source.
- **Data-driven formats (RON/YAML/TOML + JSON Schema), hot reload:** **not verified** (not researched).

## 9. Performance

- **ABM cross-framework benchmark** (JuliaDynamics CI; Agents.jl 6.2.10, Mesa 3.2.0, NetLogo 6.4.0, MASON 22.0; time relative to Agents.jl = 1.0):

| Model | MASON | NetLogo | Mesa | Ark.jl (ECS) |
|---|---|---|---|---|
| WolfSheep small | 5.29 | 9.94 | 9.38 | 0.36 |
| WolfSheep large | 7.8 | 4.81 | 3.28 | 0.17 |
| Flocking small | 1.42 | 15.37 | 159.29 | 0.74 |
| Flocking large | 0.61 | 19.14 | 59.5 | 0.36 |
| Schelling small | 1.19 | 11.39 | 29.73 | 0.85 |
| Schelling large | 1.51 | 14.33 | 26.66 | 0.99 |

Source: https://github.com/JuliaDynamics/ABMFrameworksComparison. The ECS-based Ark.jl beats the ABM frameworks by up to roughly 6x.
- **krABMaga (JASSS 27(2) 4, 2024-03-31):** claims it "always performs better" than Mesa, NetLogo, MASON, Repast and Agents.jl on Flockers, Schelling, WolfSheepGrass and ForestFire. At 128,000 agents on a 1,131×1,131 field it took about 85 s where the others reportedly crashed or used about 90% of memory. Distributed calibration ran 96x faster on 16 nodes. Source: https://www.jasss.org/27/2/4.html (table summarised; verify exact numbers).
- **FLAME GPU 2** (circles model, 1M agents, 200 steps, 2.0.0-rc): with spatial messaging, 0.944 s on V100, 0.685 s on A100, 0.551 s on H100. Brute-force messaging took 944–1,457 s. Source: https://rse.shef.ac.uk/blog/2023-08-18-benchmarking-flamegpu2-on-h100-a100-and-v100-gpus/. Spatial partitioning was worth about 1,500x.
- **Fixed timestep:** accumulator plus render interpolation; cap steps per frame to avoid the "spiral of death": https://gafferongames.com/post/fix_your_timestep/
- **Parallelism versus determinism:** Rapier says `parallel` keeps determinism; Box2D says multithreading made it much harder; Bevy's parallel scheduler is unordered by default (sources above). rayon is at 1.12.0; its reduction-order semantics are **not verified**.
- **ECS iteration numbers:** rust-gamedev/ecs_bench_suite was archived 2022-11-27 and its numeric results were not extractable: https://github.com/rust-gamedev/ecs_bench_suite. Bevy entity-iteration figures are **not verified**.
- **Not verified from sources:** SoA/archetype gains, spatial hashing, level-of-detail and variable tick rates, dirty-region diffing, wasm SIMD speedups.

## 10. Tooling

- **Versions (crates.io registry, 2026-10-01):** release-plz 0.3.169, cargo-release 1.1.6 (MSRV 1.92), cargo-semver-checks 0.50.0, cargo-deny 0.20.2, criterion 0.8.2, divan 0.1.21 (last release 2025-04-10), proptest 1.11.0, insta 1.48.0, cargo-fuzz 0.13.2.
- **release-plz:** opens a release PR from CI, writes the changelog with git-cliff, runs cargo-semver-checks for breaking-change detection, then tags and publishes: https://release-plz.dev/docs
- **crates.io trusted publishing:** docs URL returned 404; **not verified**.
- **npm trusted publishing:** OIDC from GitHub Actions, GitLab.com shared runners or CircleCI cloud; no self-hosted runners. Requires npm CLI 11.5.1+ and Node 22.14+. Provenance is generated automatically on GitHub and GitLab for public repo plus public package, not on CircleCI. Source: https://docs.npmjs.com/trusted-publishers
- **Maven Central:** OSSRH reached end of life on 2025-06-30. Publish via the Central Publisher Portal (Maven plugin, bundle upload, API) or the OSSRH Staging API compatibility layer. Gradle is covered by third-party plugins only. Snapshots are supported. Source: https://central.sonatype.org/pages/ossrh-eol/
- **MSRV pressure from dependencies:** wasm-bindgen 1.81 (two-year policy), wasm-bindgen-cli 1.86, bevy_ecs 1.95, wasmtime 1.96, rhai and insta 1.66, rayon 1.80, cel 1.86, cargo-ndk 1.86. Sources: registry `rust_version` fields and pages above.
- **Docs:** TypeDoc 0.28.20 (2026-07-05; npm). mdBook and docs.rs **not verified**.
- **Not verified:** a GitHub Actions matrix recipe covering wasm, iOS and Android targets.

## Largest unverified items
- WKWebView JIT availability for wasm in third-party apps, and whether Lockdown Mode disables wasm there.
- Current Android WebView SharedArrayBuffer and cross-origin-isolation status (sources conflict).
- Typical wasm size for a small Rust library; recent wasm-vs-JS benchmarks.
- Rain World, Equilinox, SimEarth/SimLife, Dwarf Fortress, Ecolab, EcoSim design details.
- UniFFI XCFramework packaging docs; crates.io trusted publishing.
- Schema-evolution behaviour of postcard, rkyv, MessagePack and FlatBuffers.
- Bevy ECS iteration benchmarks.
