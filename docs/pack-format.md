# Content pack and world format (0.1)

A **pack** says how an ecosystem behaves. A **world** says how one place starts. Both are JSON. Schemas are in `spec/`. A complete small example is in `packs/reference/`.

## Pack

| Field | Type | Meaning |
|---|---|---|
| `id`, `version` | string | Identify the pack. Snapshots record both |
| `stock_max` | integer, default 3 | Upper bound for soil, water and biomass |
| `need_max` | integer, default 2 | Upper bound for a need |
| `slots` | list of names | Routine slots every agent has, in resolution order |
| `terrains` | map | See below |
| `weather` | object | `pattern`: rain per tick, repeating. `variance_sides`: 0 for none; otherwise a die is rolled each tick, lowest face is -1 rain, highest is +1. `evaporation`: water lost per tick, default 1 |
| `needs` | map of name to `{ baseline }` | Level each agent starts every tick with |
| `activities` | map | `slot` and a list of effects |
| `structures` | map | `terrains` it may stand on (empty for any) and a list of effects |
| `species` | map | `requires`: list of habitat rules; `min_species`: other species that must be present; `grace`: ticks tolerated without habitat |

### Terrain

| Field | Meaning |
|---|---|
| `max_biomass` | Most plants a patch of this terrain can hold |
| `water_capacity` | Most water it can hold |
| `soil_adds_capacity` | If true, each point of soil adds a point of water capacity |
| `grows` | If true, plants grow by themselves toward the soil level when watered |

### Effect

```json
{ "target": { "kind": "nearest", "terrain": "lot", "radius": 2 }, "what": "soil", "delta": 1 }
```

The origin is the agent's home patch for an activity, or the structure's own patch.

| Target `kind` | Lands on | `what` is |
|---|---|---|
| `own` | The acting agent (activities only) | a need |
| `home` | The origin patch | `soil`, `water` or `biomass` |
| `nearest` with `terrain`, `radius` | The closest patch of that terrain within the radius; ties go to the lowest index | a stock |
| `around` with `radius`, optional `terrain` | Every other patch within the radius | a stock |
| `neighbours` with `radius` | Every other agent whose home is within the radius | a need |

Distance counts diagonals as one step. Values are clamped to their bounds; a clamped effect that changes nothing reports no change.

### Habitat rule

```json
{ "stock": "water", "min": 2, "patches": 3, "terrain": "creek" }
```

At least `patches` patches (of `terrain`, if given) must have `stock` at `min` or more.

## World

| Field | Meaning |
|---|---|
| `width`, `height` | Grid size |
| `patches` | Row by row, `width * height` entries of `{ terrain, soil, water, biomass }`; stocks default to 0 |
| `agents` | `{ id, x, y }`; the position is the agent's home patch |

## Growth rule

On a patch whose terrain `grows`: if water is at least 1 and biomass is below both the soil level and `max_biomass`, biomass rises by 1 and water falls by 1. On any patch, if water is 0 and biomass is 2 or more, biomass falls by 1.

## Validation

`understory validate --pack pack.json` checks that every name an effect, structure or species refers to exists. `Engine::new` runs the same check and also checks the world against the pack.
