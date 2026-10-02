# Architectural Digest: Biotope Repo

Status: design, October 1, 2026. Extends the Biotope MVP stack (React, R3F, XState, Dexie, OPFS, sqlocal, MapLibre, PMTiles, Capacitor).
Engine working name: **Understory** (placeholder).

## 1. What this repo is

The parent-and-child mobile app: 3D scenes, outdoor field activities, local records. It embeds the engine for its simulated scenes and uses the engine's data profile for what families record outdoors.

## 2. How the pieces fit

```
Scenario manifest (YAML)
        │ names a content pack, a starting world, a seed
        v
  Scene runtime (React + R3F)  <── query ──  Web Worker: @understory/web (WebAssembly)
        │                       ── commands ─>
        │
  Field activity (outdoors)
        │ observations, counts, photos (stay on device)
        v
  Local store (sqlocal / Dexie / OPFS)  ──  records in the data profile
        │
        ├── seeds the next scene's starting world (local species, season)
        └── optional export or contribution, behind the parent gate
```

## 3. What lives where

| Concept | Owner | Notes |
|---|---|---|
| Scene ecosystem (pond, window view, leaf litter) | Engine | One small world per scenario |
| Scenario flow, prompts, parent guidance | App | XState and Ink, as in the MVP plan |
| Field observations | App, stored locally | Shaped as data-profile records from day one |
| Turning observations into a starting world | App, using `crates/data` adapters compiled into the WebAssembly module | For example, observed birds raise those species' starting counts |
| Photos, precise location, family identity | App only | Never passed to the engine; never exported by default |
| Seed of Lovelight for Heartlight Heroes | App | A short code holding a taxon identifier and nothing else |

## 4. Layout additions

| Path | Purpose |
|---|---|
| `src/sim/` | Worker wrapper shared in shape with Heartlight Heroes |
| `src/records/` | Local record store and data-profile mapping |
| `src/share/` | Export to a data package; parent-gated contribution flow |
| `content/packs/` | One small content pack per scenario family |
| `scenarios/*.yaml` | Existing manifest, with new keys `pack`, `world`, `seed`, `tickLength` |

## 5. Mobile embedding

- **Default:** the WebAssembly module inside the Capacitor web view, single-threaded, in a Web Worker.
- **Budget:** scenes stay within the phone profile (about 32 by 32 patches, 100 agents). Pause the simulation when the app is backgrounded.
- **Fallback:** if WebAssembly is unavailable on a device, play the scene as a pre-recorded replay rendered from stored query frames.
- **Native plugin:** only if profiling shows the web view cannot hold frame rate. It would wrap the engine's Swift and Kotlin bindings as a Capacitor plugin with the same TypeScript interface.
- **Content packs are data.** They ship in the app bundle or download as data files. No downloaded code.

## 6. Records and the data profile

| Field activity | Record type | Standard it maps to |
|---|---|---|
| Window Watch, Backyard Bird Hour | Organism sighting, with effort (duration, observers) | Darwin Core with the Humboldt Extension |
| Whose Tracks?, Leaf Detective | Organism sighting with evidence type | Darwin Core |
| Pond Window | Sighting plus simple measurements (water clarity, temperature) | Darwin Core plus SOSA-style measurements |
| Future garden activities | Garden record: bed, planting, harvest | The profile's garden schema, mapped to farmOS logs |

Every record carries source type, evidence tier, a pseudonymous contributor identifier, a licence and a location precision.

## 7. Privacy rules (non-negotiable)

- Default-private. No third-party analytics. Records stay on the device.
- Location is reduced to a coarse cell on the device before any export.
- A child never contributes directly. Sharing is a parent action behind the parent gate, per record set, with a preview of exactly what leaves the device.
- No photos of people leave the device. Photos are not part of any contribution in the MVP.
- Contributions default to CC BY 4.0 with a CC0 option, and can be withdrawn.
- Get legal review of the contribution flow before it ships (COPPA, Canadian privacy law, Apple Kids Category rules).

## 8. Versioning

- Pin the engine version per app release. Scenes are short, so saved snapshots can be discarded on upgrade; local records must never be.
- Records store the data-profile version; write migrations for records, not for snapshots.

## 9. Do and do not

| Do | Do not |
|---|---|
| Store observations in profile shape from the first build | Invent app-specific record formats to convert later |
| Keep the engine blind to who the family is | Send photos, names or precise coordinates to any server by default |
| Show "this is a simulation" wherever a simulated number appears | Present simulated outcomes as predictions for a real garden |
| Reuse the worker wrapper pattern from Heartlight Heroes | Share game rules code with Heartlight Heroes through the engine |
