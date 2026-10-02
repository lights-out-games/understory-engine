# Model description (ODD protocol)

This follows the ODD protocol for describing agent-based models (Grimm et al., second update 2020), so that people who work with ecological models can see quickly what this one does and does not do. It describes engine version 0.1.

## 1. Purpose and patterns

**Purpose.** To give games and learning tools a small ecosystem that responds legibly to human routines. The model is for play and teaching. It is not for prediction, and it has not been calibrated or validated against field data.

**Patterns it should reproduce**, qualitatively: plants grow where there is soil and water and decline in drought; wildlife arrives after habitat exists and lingers briefly after it degrades; what people do daily changes the land around their homes and what they get from it.

## 2. Entities, state variables and scales

| Entity | State | Range |
|---|---|---|
| Patch (grid cell) | terrain; soil, water, biomass; optional structure | stocks are integers from 0 to `stock_max` (default 3) |
| Agent (a resident) | home patch; one level per need; one activity or none per routine slot | needs are integers from 0 to `need_max` (default 2) |
| Species | present or absent; a stress counter while habitat is missing | |
| World | tick; seed; generator state; last rain | |

**Scales.** Space and time are abstract and set by the host. In the first game a patch is a building lot and a tick is ten years. The grid is small (the first game uses 6 by 6). Distance is Chebyshev: diagonal neighbours are adjacent.

## 3. Process overview and scheduling

One tick runs these in order, each to completion:

1. Weather: evaporation, then rain up to each patch's capacity.
2. Needs reset to baseline, plus any one-tick bonus from the host.
3. Activities: agents in list order, slots in pack order.
4. Structures: patches in index order.
5. Growth and wilting.
6. Species arrival and departure.

Host commands are applied between ticks. Updates are sequential, so an earlier agent's activity can change what a later agent's activity finds.

## 4. Design concepts

- **Basic principles.** Stocks and flows on a grid, with thresholds. Limiting factors: plants are limited by soil and water; species by habitat extent and by the presence of other species.
- **Emergence.** Species presence emerges from patch states. Nothing else is emergent in 0.1; structures are placed by the host.
- **Adaptation, objectives, learning, prediction, sensing.** None. Agents do what their routine slots say. The human players are the adaptive element.
- **Interaction.** Indirect, through shared patches; direct only where an activity raises neighbours' needs.
- **Stochasticity.** One die roll per tick varies rain by -1, 0 or +1. Nothing else is random. The generator is PCG32, seeded by the host.
- **Collectives.** None.
- **Observation.** Every tick returns a list of events with each changed value's old and new level; the full state and a state hash are available at any time.

## 5. Initialization

The host supplies the grid (terrain and starting stocks per patch), the agents and their homes, and a seed. Needs start at baseline, routine slots empty, no species present.

## 6. Input data

None. The model reads no external data. Rain follows a short repeating pattern in the content pack.

## 7. Submodels

- **Water.** `water = min(capacity, max(0, water - evaporation) + rain)`. Capacity is the terrain's base capacity, plus soil where the terrain says so.
- **Growth.** If the terrain grows, water is at least 1, and biomass is below `min(max_biomass, soil)`: biomass +1, water -1. Otherwise, if water is 0 and biomass is at least 2: biomass -1.
- **Activities and structures.** Lists of effects, each adding an integer to a stock or a need at a target (see `pack-format.md`), clamped to bounds.
- **Species.** A species' habitat is met when every one of its rules is met and enough other species are present. It arrives on the first tick its habitat is met. While present, each tick without habitat adds one stress; when stress exceeds `grace` it leaves; a tick with habitat clears the stress.

## Known simplifications

No nutrient cycle beyond a single soil level; no water flow between patches; no populations, only presence; no seasons within a tick; no dispersal distance. Each is a candidate for a later version and each would be a breaking change to state hashes.
