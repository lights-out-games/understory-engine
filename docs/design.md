# Ecosystem Simulation Engine: Open-Source Design

Oct 1, 2026 · @MG

> **Status, October 2, 2026.** This is the design as written before implementation. Version 0.1 of the code implements a first slice of it; `README.md` and `docs/architecture-digest.md` say what exists and where the code differs from this document.

## Summary

Build a small deterministic Rust library, license it permissively, and grow it outward through data standards, not through a bigger simulator. Working name: Understory, a placeholder until a name search is done.

- **Licence:** MIT OR Apache-2.0 for code, CC BY 4.0 for reference content, CC0 for schemas, and a trademark on the name. Copyleft would block your own Steam and iOS releases.
- **Language:** Rust, shipped as WebAssembly for browsers, mobile web views and Node, with a C interface for native game engines. This replaces the TypeScript core in the earlier R&D strategy and adds time to its first phase.
- **Shape:** six calls, commands in and events out, content as data. Products keep their own rules in their own repositories.
- **Scale:** federate many small worlds through published data packages, with a coarse regional model on top. Do not build a distributed simulator.
- **Data:** a profile that reuses Darwin Core, Data Package and the Data Food Consortium's terms. No open standard exists for home garden records or in-person market listings; those two small schemas are the new contribution, proposed inside existing communities.
- **Biggest risks:** a two-person team taking on an engine, a standard and two products at once, and the thin supply of openly licensed ecological content.

Nothing here is built or benchmarked, and the legal sections are research to take to a lawyer.

## Scope

The engine is a deterministic, headless simulation of a small ecosystem: a library that takes a world, a rule set and a list of commands, and returns the next state and the events that occurred. It draws nothing, stores nothing and talks to no network.

| In the engine | Left to the host application |
| --- | --- |
| World grid of patches with soil, water, light and temperature | Rendering, audio, interface |
| Stocks and flows: water, nutrients, energy, biomass | Story, characters, scoring, game modes |
| Populations of species with growth, feeding, dispersal and death | Player accounts, saves, sync, networking |
| Individual agents with needs and routines (people, notable animals) | Content authoring tools and asset pipelines |
| Seasons, weather and disturbance events | Real-world data capture (photos, location) |
| Seeded randomness, snapshots, replay, state hashing | Analytics, payments, moderation |
| A command and event interface, and a query interface | Deciding what a command costs or who may send it |

Three properties define it.

- **Deterministic.** The same seed, content and commands give the same state on every platform. This is what makes replays, automated balance testing, save files and host-authoritative multiplayer cheap.
- **Data-driven.** Species, habitats, processes and parameters arrive as content packs. A product changes the ecosystem by changing data, and only writes code for behaviour the engine does not have.
- **Small-world.** It targets one garden, pond, neighbourhood or bioregion at coarse resolution. It is a model for play and learning, not for research forecasts.

Heartlight Heroes' Heartlight, Darkroots and habit cards are host rules layered on top through the extension interface. They do not belong in the core.

## Prior art

No existing library is at once an ecosystem model, deterministic, headless and shipped for web, mobile and native. That absence comes from a search, not a proof, but every near neighbour misses at least one of the four.

| Project | Language and licence | Why it does not fill the slot |
| --- | --- | --- |
| [krABMaga](https://github.com/krABMaga/krABMaga) | Rust, MIT | General agent framework with a browser visualiser; no ecology model and no cross-platform determinism claim. The nearest relative and worth studying |
| [AgentScript](https://github.com/backspaces/agentscript) | JavaScript, GPL-3.0 | JavaScript only, and copyleft |
| [NetLogo](https://github.com/NetLogo/NetLogo) | JVM, GPL-2.0 | Desktop application; the standard teaching tool, not an embeddable library |
| [Mesa](https://github.com/projectmesa/mesa), [Agents.jl](https://github.com/JuliaDynamics/Agents.jl) | Python, Julia | Tied to their language runtimes |
| [iLand](https://github.com/edfm-tum/iland-model), [LPJ-GUESS](https://zenodo.org/records/8065737), LANDIS-II | C++ and C# | Research models that need external datasets and desktop or cluster runtimes |
| [FLAME GPU 2](https://github.com/FLAMEGPU/FLAMEGPU2) | CUDA, AGPL-3.0 | Requires an NVIDIA GPU |
| [APSIM](https://www.apsim.info/download-apsim/) | C# | Not open source; commercial use is fee-licensed |

Three lessons carry over. Eco simulates thousands of plants and animals individually, and pays for it with an always-on server; an embeddable engine has to offer coarser representations instead. Entity-component storage is fast: in one [cross-framework benchmark](https://github.com/JuliaDynamics/ABMFrameworksComparison) an ECS implementation beat the agent frameworks by up to about six times. And [Box2D](https://box2d.org/posts/2024/08/determinism/) and [Rapier](https://rapier.rs/docs/user_guides/rust/determinism/) show that cross-platform determinism with ordinary floating point is achievable when the math functions are controlled.

## Licensing and legal

License the code under MIT OR Apache-2.0, the reference content under CC BY 4.0, and the data schemas under CC0; hold the name as a trademark outside all three. This section is research, not legal advice, and a lawyer should review it before the first public release.

**Code licence options**

| Licence | Third parties can ship closed products | iOS and Steam | Engine changes come back | Verdict |
| --- | --- | --- | --- | --- |
| MIT OR Apache-2.0 | Yes | No conflict; [Valve names both as fine](https://partner.steamgames.com/doc/sdk/uploading/distributing_opensource) | No | **Recommended.** The [Rust convention](https://rust-lang.github.io/api-guidelines/necessities.html); Apache adds a patent grant |
| MPL-2.0 | Yes; [static linking into proprietary software is allowed](https://www.mozilla.org/en-US/MPL/2.0/FAQ/) | Works on iOS (Firefox uses it) | Yes, per modified file | Reasonable alternative if you want fixes returned |
| LGPL | Only with relinking rights | Hard to satisfy in a single WebAssembly or iOS binary | Yes | Avoid |
| GPL or AGPL | No | Valve calls copyleft "problematic" with the Steamworks SDK; Apple removed GPL apps in the past | Yes | Avoid; it blocks your own products |
| Source-available (BSL, FSL, Elastic) | With limits | Fine | Varies | Avoid; [not open source by OSI's definition](https://opensource.org/blog/the-sspl-is-not-an-open-source-license) |

The permissive choice matches the goal that other creators own what they build. It is also what [Godot](https://godotengine.org/license/), [Bevy](https://github.com/bevyengine/bevy), Box2D and [Rapier](https://github.com/dimforge/rapier) use. The cost is that a company can fork the engine privately and return nothing.

**Other legal decisions**

| Topic | Decision | Why |
| --- | --- | --- |
| Contributions | [Developer Certificate of Origin](https://developercertificate.org/) sign-off; no contributor agreement | Lower friction; [GitLab dropped its agreement](https://about.gitlab.com/blog/gitlab-switches-to-dco-license/) because it deterred contributors. It also means the licence cannot be tightened later without every contributor's consent, which is a promise to adopters |
| Name and logo | Registered trademark with a short usage policy; forks must rename | The code licence grants no trademark rights, as in [Mozilla's policy](https://www.mozilla.org/en-US/foundation/trademarks/policy/). USPTO filing is [USD 350 per class](https://www.uspto.gov/trademarks/trademark-fee-information) |
| Content packs | Reference packs CC BY 4.0; third parties choose their own | Creative Commons [advises against its licences for software](https://creativecommons.org/faq/), so data and code stay in separate folders with separate licences |
| Schemas and the data profile | CC0 | A standard nobody has to ask permission to implement |
| Home for the project | Lights Out Games at first; move the name and repository to a fiscal host once outside contributors arrive | [Open Source Collective](https://docs.oscollective.org/welcome-and-introduction-to-osc/fees) takes 10% and can hold trademarks. [Software Freedom Conservancy](https://sfconservancy.org/projects/apply/) wants a year of history and a diverse community |
| Dependencies | Permissive only, enforced in CI with [cargo-deny](https://embarkstudios.github.io/cargo-deny/checks/licenses/index.html); [REUSE](https://reuse.software/spec-3.3/) headers on every file | Lets adopters generate their in-app licence screen automatically |

**Rules that apply to products, not to the library**

- **EU Cyber Resilience Act.** Open-source code published outside a commercial activity is out of scope, but whoever sells a product built on it is the manufacturer. Lights Out Games is the manufacturer for Heartlight Heroes and Biotope. [Full application is December 11, 2027](https://digital-strategy.ec.europa.eu/en/policies/cra-summary).
- **Children's privacy.** The engine collects nothing and makes no network calls, so it adds no [COPPA](https://www.ftc.gov/business-guidance/resources/complying-coppa-frequently-asked-questions) burden. The app that embeds it remains the operator.
- **Apple guideline 2.5.2.** Apps may not download code that changes their features. Content packs must therefore be data, never scripts, on iOS.
- **Data licences.** Most of the legal risk sits in content, not code. Non-commercial sources cannot go into a paid product or a market seller's listing; the data standards section lists which sources are safe.

## Language and tooling

Write the core in Rust and ship it three ways: WebAssembly for anything with a JavaScript runtime, a C interface for native game engines, and the plain crate for Rust servers. This replaces the TypeScript simulation core named in the Heartlight Heroes R&D strategy, and it is the largest cost in this design.

| Option | Reaches web, mobile, native engines, servers | Determinism | Cost for a two-person team |
| --- | --- | --- | --- |
| **Rust** | All four from one codebase; every target is [Tier 2](https://doc.rust-lang.org/nightly/rustc/platform-support.html) | Controllable; proven by Rapier and Box2D-style discipline | Slower iteration than TypeScript; needs Rust fluency |
| TypeScript only | Web, Capacitor, Node. Unity, Godot and Unreal hosts would have to embed a JavaScript engine | Achievable, with care around floats | Lowest; matches the current stack and agent workflow |
| C or C++ | All four through Emscripten and native builds | Controllable | Memory safety and build tooling burden |
| C# | Native AOT on iOS and Android is still [experimental](https://learn.microsoft.com/en-us/dotnet/core/deploying/native-aot/) | Proven by commercial deterministic engines | Weak web story |
| Zig, AssemblyScript, Kotlin Multiplatform | Partial; Zig is pre-1.0, AssemblyScript is WebAssembly-only, Kotlin/Wasm is beta | Unproven here | Tooling risk |

If the engineer does not want to carry Rust on nights and weekends, the fallback is a TypeScript core behind the same command and event interface, ported later. The interface in this document is designed so that swap does not touch the host applications.

**Bindings**

| Host | Binding | Status |
| --- | --- | --- |
| Browser, Capacitor, Electron, Tauri, Node | [wasm-bindgen](https://github.com/wasm-bindgen/wasm-bindgen/releases) 0.2.x, with generated TypeScript types | Maintained under a new organisation after the [2025 rustwasm archive](https://blog.rust-lang.org/inside-rust/2025/07/21/sunsetting-the-rustwasm-github-org) |
| Unity, Godot, Unreal, any C host | C interface generated with cbindgen; [csbindgen](https://github.com/Cysharp/csbindgen) for Unity | Stable pattern; all three engines load C libraries |
| Native Swift and Kotlin | [UniFFI](https://mozilla.github.io/uniffi-rs/latest/) 0.32 | Still 0.x; packaging is on you. Defer until a host needs threads on mobile |
| Rust servers | The crate itself | No binding |

**Determinism rules, enforced in CI**

- Fixed tick; the host never passes wall-clock time into the engine.
- No standard-library transcendental math. Rust documents `sin`, `exp`, `powf` and others as [varying by platform](https://doc.rust-lang.org/std/primitive.f64.html); use a pure-Rust math crate and a small fixed set of functions.
- A named random generator (PCG or ChaCha) with one stream per system. The `rand` project [warns](https://rust-random.github.io/book/crate-reprod.html) that its default generators are not portable.
- No iteration over hash maps; ordered or indexed collections only.
- No relaxed SIMD, whose results [vary by hardware](https://github.com/WebAssembly/relaxed-simd/blob/main/proposals/relaxed-simd/Overview.md). Canonicalise NaN before hashing state.
- A golden test: the same seed and command log must produce the same state hash on native x86, native ARM, and WebAssembly in Chrome, Firefox and Safari.

**Tooling**

| Need | Tool |
| --- | --- |
| Workspace and releases | Cargo workspace, [release-plz](https://release-plz.dev/docs) with semver checks |
| Storage | Plain structure-of-arrays tables in the core. Not `bevy_ecs`, whose default system order is [nondeterministic](https://bevy-cheatbook.github.io/programming/system-order.html) and whose minimum Rust version moves fast |
| Snapshots | [postcard](https://docs.rs/postcard/latest/postcard/), with an explicit version header. Not bincode, which is [unmaintained](https://osv.dev/vulnerability/RUSTSEC-2025-0141) |
| Content packs | JSON with JSON Schema, so non-Rust tools can validate them |
| Tests | Property tests for conservation laws (water and nutrients in equal out), snapshot tests, fuzzing of the command interface, benchmarks tracked per commit |
| Publishing | crates.io, npm with [trusted publishing](https://docs.npmjs.com/trusted-publishers), prebuilt C libraries on GitHub releases |

## Architecture

The core is one Rust library with no input or output of its own; hosts reach it through a binding, and everything that makes a product different arrives as data.

```
 Heartlight Heroes     Biotope             Other creators        Servers
 browser, Steam        Capacitor app       Unity, Godot, Unreal  regional, research
        |                   |                     |                    |
        v                   v                     v                    v
 +--------------------------------------+  +-------------+  +-------------+
 | WebAssembly module + TypeScript types |  | C interface |  | Rust crate  |
 +--------------------------------------+  +-------------+  +-------------+
                    |                             |                |
                    v                             v                v
 +------------------------------------------------------------------------+
 | Engine core: Rust, deterministic, no input or output of its own         |
 |  World            Life              Processes     Interface    State     |
 |  patches, soil,   populations,      ordered       commands,    snapshot, |
 |  water, light     agents, routines  systems,      events,      replay,   |
 |                                     weather       queries      hash      |
 +------------------------------------------------------------------------+
                    ^                                      ^
                    |                                      |
 +--------------------------------------+  +--------------------------------+
 | Content packs: JSON, data only        |  | Data adapters: optional crate  |
 | species, habitats, parameters, worlds |  | Data Package, Darwin Core, H3  |
 +--------------------------------------+  +--------------------------------+
```

Hosts sit above the core and own everything players see. Content and real-world data enter from below, as files the host loads and hands over.

**The interface every host uses**

| Call | Purpose |
| --- | --- |
| `create(pack, world, seed)` | Build a world from a content pack, a starting world and a seed |
| `apply(commands)` | Queue host actions for the next tick: plant, remove, set an agent's routine, change a parameter, trigger a disturbance |
| `step(ticks)` | Advance the simulation; returns the events that occurred |
| `query(view)` | Read state as flat typed arrays for rendering: a patch layer, a population table, agents in a region |
| `snapshot()` and `restore(bytes)` | Save and load the full state |
| `hash()` | A checksum of the state, for replay tests and multiplayer agreement |

A save file or a replay is the seed, the pack version and the command log. Two devices that apply the same commands at the same ticks hold the same world, which is what makes host-authoritative play, automated balance runs and bug reports cheap.

**Repository layout**

| Path | Contents | Licence |
| --- | --- | --- |
| `crates/core` | World, life, processes, scheduler, random streams, snapshots | MIT OR Apache-2.0 |
| `crates/content` | Pack schema, loader, validation | MIT OR Apache-2.0 |
| `crates/data` | Import and export adapters for the data profile | MIT OR Apache-2.0 |
| `crates/wasm`, `crates/ffi-c` | Bindings | MIT OR Apache-2.0 |
| `crates/cli` | Run, replay, hash, benchmark, validate a pack | MIT OR Apache-2.0 |
| `packages/web` | npm package: TypeScript types and a Web Worker wrapper | MIT OR Apache-2.0 |
| `packs/reference` | A small temperate garden and pond | CC BY 4.0 |
| `spec/` | Data profile and JSON Schemas | CC0 |
| `docs/` | Guide, and a model description in the [ODD format](https://www.jasss.org/23/2/7.html) that scientists expect | CC BY 4.0 |

## Integration targets

One WebAssembly build covers the browser, mobile web views, desktop wrappers and Node; native bindings are added only where a host cannot run WebAssembly well.

| Target | How the engine is embedded | Threads | Main constraint |
| --- | --- | --- | --- |
| Browser | WebAssembly in a Web Worker; the render thread reads query results | One simulation thread. Shared-memory threads need [cross-origin isolation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/SharedArrayBuffer), which portals and embeds often cannot set | Keep each query result small; copying is the cost |
| Mobile, Capacitor | The same WebAssembly module inside the web view | One. Shared memory is [unreliable in web views](https://github.com/ionic-team/capacitor/issues/6182) | Memory and battery. Whether Apple's Lockdown Mode disables WebAssembly in web views is unconfirmed; ship a "simulation unavailable" fallback |
| Mobile, native | Swift and Kotlin bindings through UniFFI, packaged as a Capacitor plugin | Native threads | Extra build and packaging work; only when the web view is measurably too slow |
| Desktop wrapper (Electron, Tauri, Steam) | Same as browser | One, or more with isolation set by the wrapper | None beyond browser |
| Native game engines | C library with generated headers; csbindgen for Unity, GDExtension for Godot | Host decides | Third parties maintain their own wrappers; [Godot's Rust bindings](https://github.com/godot-rust/gdext) call mobile and web "experimental" |
| Node server | WebAssembly module, identical bits to the client | One per world; many worlds per process | Good for validation and replays |
| Rust server | The crate, compiled natively | Deterministic parallel regions | Fastest; the path for large worlds |
| Edge functions | WebAssembly | One | [128 MB per isolate](https://developers.cloudflare.com/workers/platform/limits) on Cloudflare Workers, including the JavaScript heap |

**Public and private servers**

A server host is a thin program that owns networking and storage and calls the same six functions. Three shapes are supported.

- **Validator.** Receives a seed and a command log, replays it, and confirms the hash. Used for leaderboards, anti-tamper checks and bug reports. Stateless and cheap.
- **World host.** Keeps one long-running world per classroom, garden club or game session, and streams state changes to clients. A private server is this, run by a school or studio on its own machine.
- **Regional host.** Runs a coarse model over many contributed worlds. Described under Scaling up.

The engine ships no server. It ships a reference world host in `crates/cli` small enough to read in an afternoon, so that companies build and own their own.

## Customization and performance tuning

Products customize the engine at four levels, each more powerful and more costly than the last; most products should never leave the first two.

| Level | What changes | Who does it | Works on iOS | Keeps determinism |
| --- | --- | --- | --- | --- |
| 1. Content pack | Species, habitats, parameters, starting worlds, which built-in processes run and how often | Designers, in JSON | Yes | Yes |
| 2. Host rules | Game logic between ticks: read events, decide, send commands. Heartlight, scoring and missions live here | Product engineers, in the host language | Yes | Yes, if the host logs its commands |
| 3. Compiled process | A new process written in Rust against the `Process` trait and compiled into a custom build | Engine-level contributors | Yes | Yes, if it follows the determinism rules |
| 4. Plugin module | A sandboxed WebAssembly process loaded at run time | Server operators and researchers | No; downloaded code is barred by guideline 2.5.2 | Only with a deterministic runtime configuration |

Level 4 is deferred. It is the only level with unsolved problems, and nothing in either product needs it.

**Tuning controls**

| Control | Effect | Typical use |
| --- | --- | --- |
| Grid size and patch scale | Memory and per-tick cost scale with patch count | A phone scene uses a small grid; a server region uses a large one |
| Tick length | Simulated time per step | Hours for a pond scene, seasons for a 100-year session |
| Process cadence | Each process runs every N ticks | Soil chemistry weekly, animal movement hourly |
| Representation level | A species is tracked as a population count per patch or as individual agents | Individuals only where the player can see them |
| Agent cap | Hard limit with a defined overflow rule | Keeps worst-case frame time bounded on phones |
| Tick budget | The host asks for as many ticks as fit in a time slice | Smooth frame rate while fast-forwarding |
| Query shape | Full layer, changed patches only, or a region | Rendering reads only what moved |
| Parallel regions | Native builds only; regions step in parallel with a fixed merge order | Servers |

**Performance targets, to be measured**

These are design targets, not results. The first benchmark suite exists to confirm or correct them.

| Profile | World | Target |
| --- | --- | --- |
| Phone scene | 32 by 32 patches, 100 agents, 20 species | Under 1 millisecond per tick in a web view |
| Game session | 64 by 64 patches, 500 agents, 60 species | 100 simulated years in under 2 seconds headless |
| Server region | 1,024 by 1,024 patches, 50,000 agents | Real-time at one tick per second on four cores |

For scale, the Rust framework krABMaga [reports](https://www.jasss.org/27/2/4.html) simple models with 128,000 agents on one machine, so the server target is conservative for individuals and untested for the patch grid.

## Limitations and best-fit uses

The engine fits small, legible ecosystems that people play with or learn from; it is the wrong tool for prediction, physics or massive single worlds.

| Good fit | Poor fit |
| --- | --- |
| Games where an ecosystem responds to player choices | Research forecasts or anything used for real land-management decisions |
| Classroom and family learning scenes: a pond, a garden bed, a hedgerow | Continuous physics: fluid flow, terrain erosion, individual plant geometry |
| "What if" garden planning at the level of beds and seasons | Crop yield prediction; use [DSSAT](https://github.com/DSSAT/dssat-csm-os) or AquaCrop for that |
| Shared worlds that must replay identically on every device | Millions of individuals in one world on one device |
| Regional pictures assembled from many small contributed worlds | Real-time action games that need per-frame simulation of every creature |

**Known limitations**

- **Single-threaded in browsers and web views.** Speed there comes from doing less, not from more cores.
- **Determinism has a price.** A restricted math set, no host floats inside rules, and stricter code review for every process.
- **Coarse by design.** Patches are metres to hundreds of metres wide. Outputs are plausible, not validated; the documentation must say so wherever a number is shown to a learner.
- **No scripting on iOS.** Behaviour that is not in the engine build cannot be downloaded later.
- **Snapshots are version-bound.** A save from one engine version needs a migration to load in the next major version; replays need the exact version.
- **Content quality is the real ceiling.** A believable ecosystem needs good species and interaction data, and openly licensed data of that kind is thin.

## Scaling up

Scale by federating many small worlds through shared data, not by building one giant distributed world. The core stays the same at every tier; only the host around it changes.

| Tier | What runs | How worlds connect | Build it when |
| --- | --- | --- | --- |
| 1. Local world | One world on one device | Not connected | Now |
| 2. Large world | One world on a native server, split into regions that step in parallel | Border cells copied between neighbouring regions each tick, merged in a fixed order | A product needs a world too big for one core |
| 3. Federation | Thousands of independent local worlds plus a coarse regional model | Each world publishes a periodic summary as a data package; the regional model returns context such as weather, pollinator pressure and seed arrival | Citizen-science and education use begins |
| 4. Research coupling | The engine as one component beside professional models | A standard model-coupling interface, and gridded files as inputs | A research partner asks |

**Why federation and not a distributed simulation**

- **Community data is late, uneven and offline.** Gardeners report weekly at best. A design that needs every node in step would never run.
- **Determinism survives.** Each local world replays exactly. The regional layer is statistical and makes no such promise.
- **Nobody has to run a cluster.** A regional host is an ordinary server reading files.
- **The distributed route is a research field of its own.** No framework reviewed combines determinism, a browser runtime and partitioning across machines.

**What exists at the large end**

| Project | Scale and approach | Lesson |
| --- | --- | --- |
| [krABMaga distributed](https://ceur-ws.org/Vol-3785/paper124.pdf) | Rust and MPI; 1 to 10 million agents on 128 cores, 93 times faster than one core; no load balancing | Region partitioning with border cells is the proven pattern for tier 2 |
| [TeraAgent](https://arxiv.org/abs/2509.24063) | Extends BioDynaMo; reports half a trillion agents | The ceiling is far above anything these products need |
| [Repast4Py](https://pypi.org/project/repast4py/) | Python on MPI, BSD licence, still beta | Where a research group would go instead; do not compete |
| [BioDT](https://biodt.eu/) | EU biodiversity digital twins on the LUMI supercomputer, including bird monitoring from citizen-science data | Citizen data feeding ecological models is established practice. The project appears to have concluded in April 2025 |
| [Destination Earth](https://destination-earth.eu/) | EU digital twins of climate and weather extremes | A future source of regional context layers |
| [Factorio](https://wiki.factorio.com/Multiplayer) | Every player simulates the whole world; only inputs are sent | The model for tier 1 multiplayer |
| [EVE Online](https://www.eveonline.com/news/view/introducing-time-dilation-tidi) | Slows the game clock under load | A regional host may simply run slower than real time |

**Building blocks to adopt, not invent**

| Need | Choice | Status |
| --- | --- | --- |
| A global address for any place | [H3](https://github.com/uber/h3) hexagonal cells, Apache-2.0 | Mature. OGC also has an approved [grid-system standard](https://www.ogc.org/standards/dggs/) |
| Large tables | GeoParquet | Version 2.0 is a [release candidate](https://geoparquet.org/) |
| Gridded layers such as soil and climate | [Zarr v3](https://zarr-specs.readthedocs.io/en/latest/v3/core/index.html) | Stable; the geospatial conventions are [still draft](https://github.com/zarr-developers/geozarr-spec) |
| Map display | [PMTiles](https://docs.protomaps.com/pmtiles/) | Already in the Biotope stack; read-only |
| Catalogue of published packages | [STAC 1.1](https://github.com/radiantearth/stac-spec/releases) | Stable |
| Coupling with research models | [BMI 2.0](https://bmi.csdms.io/en/stable/) through the C interface | No Rust binding is listed; the C one suffices |
| Records edited on several devices | A merge-friendly document library such as [Automerge](https://automerge.org/blog/automerge-3/) or [Loro](https://github.com/loro-dev/loro) | For user records only, never for simulation state |

Explicitly out of scope: MPI partitioning, the defence-oriented HLA standard, and live synchronisation of simulation state between strangers' devices.

## Data standards

Do not write a new standard. Publish a profile: a documented way to combine existing standards inside one data package, plus two small record types for the places where nothing exists, which are garden records and market listings.

**The profile, layer by layer**

| Record | Existing standard to reuse | Its status | What the profile adds |
| --- | --- | --- | --- |
| Package and tables | [Data Package v2](https://datapackage.org/) | Released June 2024; the base of the two biodiversity packages below | A fixed set of table names and a manifest |
| Organism sightings and surveys | [Darwin Core](https://www.tdwg.org/standards/dwc/) with the Humboldt Extension; the new [Darwin Core Data Package](https://www.tdwg.org/news/2026/dwc-additions-ratified/) | Data Package guide ratified by TDWG in 2026 | Nothing; use as is so records can flow to GBIF |
| Measurements: soil tests, rainfall, temperature | [SOSA/SSN](https://www.w3.org/TR/vocab-ssn-2023/) terms and [STAplus](https://docs.ogc.org/is/22-022r1/22-022r1.html) | SOSA 2023 edition is still a draft; STAplus is an OGC standard that adds contributor, licence and campaign | A short list of allowed properties and units |
| Projects and participants | [PPSR Core](https://core.citizenscience.org/) | Documentation is still alpha | Minimal subset |
| Species identity | GBIF and [Wikidata](https://www.wikidata.org/wiki/Wikidata:Licensing) identifiers | Stable, CC0 | One required identifier per taxon |
| Place | H3 cell, with optional GeoJSON outline | Stable | A required precision level |
| **Garden records**: beds, plantings, harvests, inputs | None. Nearest: the [farmOS](https://farmos.org/model/) asset and log model, [Growstuff](https://github.com/Growstuff/growstuff)'s plantings and harvests, [Farming Concrete](https://farmingconcrete.org/toolkit/)'s 20 measurement methods | farmOS conventions are [informal by its own description](https://farmos.org/model/convention/); the others are one application each | A new, small schema that maps onto farmOS logs |
| **Market listings**: seller, product, offer, origin | [Data Food Consortium](https://github.com/datafoodconsortium/ontology) ontology; schema.org Product and Offer; optional [PLU codes](https://www.ifpsglobal.com/plu-codes) | Built for online food platforms such as Open Food Network; nothing covers in-person market stalls | A new, small schema that maps onto the consortium's terms |
| Simulation output | None |  | Engine summaries per cell, always flagged as simulated |

**Fields every record carries**

- **Source type:** observed, measured, self-reported, estimated or simulated. Simulated and observed values never share a column.
- **Evidence tier:** self-reported, with photo or weight, community-confirmed, or instrument and professional. iNaturalist's quality model marks cultivated organisms as ["Casual"](https://help.inaturalist.org/en/support/solutions/articles/151000169936-what-is-the-data-quality-assessment-and-how-do-observations-qualify-to-become-research-grade-), so garden data needs its own tiers.
- **Contributor:** a pseudonymous identifier, never a name.
- **Licence:** a standard licence identifier per record.
- **Location precision:** the cell size the contributor agreed to publish.

**Privacy and ownership rules built into the profile**

- **Home locations are generalised before they leave the device.** [GBIF's guidance](https://docs.gbif.org/sensitive-species-best-practices/master/en/) prefers coarsening over random offsets and says to record that it was done.
- **Children contribute through a parent.** Photos and precise location are personal information under COPPA, and iNaturalist itself requires users to be [13 or older](https://www.inaturalist.org/pages/terms).
- **Contributors own their records** and can export or delete them, following the [farm-data principles](https://www.agdatatransparent.com/principles) the agriculture industry already uses.
- **A field for community labels,** so Indigenous knowledge can carry [Local Contexts](https://localcontexts.org/labels/about-the-labels/) notices.
- **Default licence CC BY 4.0, with CC0 offered.** Not non-commercial and not share-alike, because a market seller is a commercial user and share-alike would bind every dataset mixed with it.

**Which existing data can go into a paid product**

| Usable with attribution or freely | Share-alike: keep in a separate package | Not usable commercially without permission |
| --- | --- | --- |
| [USDA PLANTS](https://plants.sc.egov.usda.gov/DocumentLibrary/Pdf/PLANTS_Help_Document_2022.pdf) data, [FoodData Central](https://fdc.nal.usda.gov/api-guide), [SoilGrids](https://docs.isric.org/globaldata/soilgrids/SoilGrids_faqs_02.html), ERA5, [Open-Meteo](https://open-meteo.com/en/licence), [ESA WorldCover](https://esa-worldcover.org/en/data-access), [Ecoregions 2017](https://ecoregions.appspot.com/), [AusTraits](https://austraits.org/), Open Tree of Life, Wikidata, [USA-NPN](https://www.usanpn.org/terms), [CoCoRaHS](https://www.cocorahs.org/Content.aspx?page=datausagepolicy), GBIF datasets marked CC0 or CC BY | [OpenStreetMap](https://osmfoundation.org/wiki/Licence/Licence_and_Legal_FAQ), [Open Food Facts](https://world.openfoodfacts.org/terms-of-use), Growstuff data, Farming Concrete data | iNaturalist records by default, [eBird](https://www.birds.cornell.edu/home/ebird-data-access-terms-of-use/), [IUCN Red List](https://www.iucnredlist.org/terms/terms-of-use), FishBase, [One Earth Bioregions](https://www.oneearth.org/bioregions-2023/), [Budburst](https://budburst.org/), Weather Underground stations |

One Earth's bioregion map is non-commercial, which matters for Heartlight Heroes: build its bioregions from the CC BY Ecoregions 2017 dataset instead.

**Gaps this work could fill, and who to do it with**

- **No open standard exists for home or community garden records,** and none for in-person market listings.
- **The main open crop-knowledge commons, OpenFarm, [shut down in April 2025](https://github.com/openfarmcc/OpenFarm).** Its data was CC0 and can seed a reference pack.
- **Propose the two new schemas inside existing communities,** not alone: [OpenTEAM](https://openteam.community/) for garden and farm records, the Data Food Consortium for listings, and TDWG for the link to Darwin Core. A standard with one implementer is a file format.

## Ownership and governance

Other creators and companies own everything they build on the engine; the project owns only the shared code, the schemas and the name.

| Asset | Who owns it | Under what terms |
| --- | --- | --- |
| Engine code | Its contributors, collectively | MIT OR Apache-2.0; anyone may use, modify, sell and fork |
| A company's fork or custom build | That company | May stay private; must keep licence notices and use its own name |
| Products built on the engine | Their makers | No royalty, no reporting, no approval |
| Content packs made by others | Their authors | Any licence they choose, including proprietary |
| Reference content packs | The project | CC BY 4.0 |
| Data profile and schemas | Public domain | CC0 |
| Records contributed by gardeners and sellers | Each contributor | Their chosen open licence; exportable and deletable |
| Project name and logo | Lights Out Games at first, then a neutral host | Trademark policy: "built with" is free; using the name for a fork or product is not |

**How governance grows**

1. **Now:** maintainer-led. Two maintainers, a public roadmap, a code of conduct, decisions recorded in the repository.
2. **When a second organisation ships a product on it:** move the trademark and repository to a fiscal host, and add an outside maintainer. Neutral ownership is what larger adopters look for; the [Linux Foundation](https://www.linuxfoundation.org/projects/hosting) makes it a hosting condition.
3. **If it grows further:** a small foundation, as [Bevy](https://bevy.org/foundation/) and [Godot](https://godot.foundation/) did. Donations become tax-deductible only with charitable status, your own or a host's.

**Paying for it** without compromising the licence: sponsorship, paid integration and support work, paid content packs and hosted regional services built on the open core. Grants are thin for a new project: [Sovereign Tech Fund](https://www.sovereign.tech/programs/fund) wants established base technology, and [NLnet's commons fund](https://nlnet.nl/commonsfund/) closed its final call in June 2026. The education-research grant route in the R&D strategy remains the better fit.

## Repo integration and build order

Three repositories, one direction of dependency: both products depend on the engine, and the engine knows nothing about either. Each repository has an architectural digest, delivered as a markdown file to drop into its `docs/` folder.

| Repository | Digest file | The one rule |
| --- | --- | --- |
| Engine | `architecture-digest-engine.md` | No product concepts in the core |
| Heartlight Heroes | `architecture-digest-heartlight-heroes.md` | If a biologist would recognise it, it belongs in the engine; if only a player would, it belongs in the game |
| Biotope | `architecture-digest-biotope.md` | The engine never sees who the family is |

**Build order**

1. **Interface and determinism harness.** The six calls, the command and event types, and the golden-hash test running on native and WebAssembly, with a trivial world. This is the part that must be right.
2. **World and three processes.** Water, plant growth and one consumer, with conservation tests. Enough for the Heartlight Heroes digital toy.
3. **Agents and content pack schema.** Residents with needs and routines; packs validated by schema.
4. **Data profile, first version.** Sightings and measurements only, with export from a Biotope scene.
5. **Public release 0.1.** Documentation, model description, trademark filing, contribution rules.
6. **Later, on demand:** C interface, garden and market schemas with partner communities, regional host.

Steps 1 to 3 sit inside Phase 1 of the Heartlight Heroes roadmap and will lengthen it. Keep the repository public from the first commit but unannounced until step 5.

## Sources

Facts were taken from the linked pages on October 1, 2026, through an automated reader; check exact wording before quoting. Not confirmed from a primary source: the LGPL and GPL texts (the site did not load), Apple Lockdown Mode's effect on WebAssembly, current shared-memory support in Android web views, and typical WebAssembly binary sizes.
