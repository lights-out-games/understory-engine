//! Data model: the content pack (rules as data) and the world (state).
use crate::rng::Rng;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

fn d1() -> i32 {
    1
}
fn d2() -> i32 {
    2
}
fn d3() -> i32 {
    3
}

/// A content pack: everything that makes one ecosystem different from another.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Pack {
    pub id: String,
    pub version: String,
    /// Upper bound for soil, water and biomass on a patch.
    #[serde(default = "d3")]
    pub stock_max: i32,
    /// Upper bound for an agent's need level.
    #[serde(default = "d2")]
    pub need_max: i32,
    /// Routine slot names every agent has, in resolution order.
    pub slots: Vec<String>,
    pub terrains: BTreeMap<String, Terrain>,
    pub weather: Weather,
    pub needs: BTreeMap<String, NeedDef>,
    #[serde(default)]
    pub activities: BTreeMap<String, Activity>,
    #[serde(default)]
    pub structures: BTreeMap<String, StructureDef>,
    #[serde(default)]
    pub species: BTreeMap<String, SpeciesDef>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Terrain {
    pub max_biomass: i32,
    pub water_capacity: i32,
    /// When true, each point of soil adds one point of water capacity.
    #[serde(default)]
    pub soil_adds_capacity: bool,
    /// When true, biomass grows by itself up to the soil level.
    #[serde(default)]
    pub grows: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Weather {
    /// Rain per tick, repeated in a cycle.
    pub pattern: Vec<i32>,
    /// 0 = no randomness. Otherwise a die with this many sides is rolled each
    /// tick: lowest face is a dry spell (-1), highest is a wet spell (+1).
    #[serde(default)]
    pub variance_sides: u32,
    #[serde(default = "d1")]
    pub evaporation: i32,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct NeedDef {
    /// Level every agent starts each tick with, before activities.
    #[serde(default)]
    pub baseline: i32,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Activity {
    pub slot: String,
    #[serde(default)]
    pub effects: Vec<Effect>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct StructureDef {
    /// Terrains it may stand on. Empty = anywhere.
    #[serde(default)]
    pub terrains: Vec<String>,
    #[serde(default)]
    pub effects: Vec<Effect>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Effect {
    pub target: Target,
    /// A need name (agent targets) or `soil` / `water` / `biomass` (patch targets).
    pub what: String,
    pub delta: i32,
}

/// Who or what an effect lands on. The origin is the agent's home patch for
/// an activity, or the structure's own patch for a structure.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum Target {
    /// The acting agent's own need. Not valid for structures.
    Own,
    /// The origin patch.
    Home,
    /// The closest patch of a terrain within `radius` (ties: lowest index).
    Nearest { terrain: String, radius: i32 },
    /// Every other patch within `radius`, optionally of one terrain.
    Around {
        radius: i32,
        #[serde(default)]
        terrain: Option<String>,
    },
    /// Needs of every other agent whose home is within `radius`.
    Neighbours { radius: i32 },
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct SpeciesDef {
    pub requires: Vec<HabitatReq>,
    /// Other species that must already be present.
    #[serde(default)]
    pub min_species: u32,
    /// Ticks of unsuitable habitat the species tolerates before it leaves.
    #[serde(default)]
    pub grace: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct HabitatReq {
    pub stock: String,
    pub min: i32,
    /// Number of patches that must meet the minimum.
    pub patches: u32,
    #[serde(default)]
    pub terrain: Option<String>,
}

/// Starting world supplied by the host.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct WorldDef {
    pub width: i32,
    pub height: i32,
    /// Row-major, `width * height` entries.
    pub patches: Vec<PatchDef>,
    #[serde(default)]
    pub agents: Vec<AgentDef>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct PatchDef {
    pub terrain: String,
    #[serde(default)]
    pub soil: i32,
    #[serde(default)]
    pub water: i32,
    #[serde(default)]
    pub biomass: i32,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct AgentDef {
    pub id: String,
    pub x: i32,
    pub y: i32,
}

/// Full simulation state. Serialises to stable, ordered JSON.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct World {
    pub tick: u32,
    pub seed: u32,
    pub rng: Rng,
    pub width: i32,
    pub height: i32,
    pub last_rain: i32,
    pub rain_override: Option<i32>,
    pub patches: Vec<Patch>,
    pub agents: Vec<Agent>,
    pub species: BTreeSet<String>,
    /// Consecutive ticks each present species has lacked its habitat.
    pub stress: BTreeMap<String, u32>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Patch {
    pub x: i32,
    pub y: i32,
    pub terrain: String,
    pub soil: i32,
    pub water: i32,
    pub biomass: i32,
    pub structure: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Agent {
    pub id: String,
    pub x: i32,
    pub y: i32,
    pub needs: BTreeMap<String, i32>,
    /// One-tick need bonus queued by the host (`adjust_need`).
    pub bonus: BTreeMap<String, i32>,
    pub slots: BTreeMap<String, Option<String>>,
}

/// A host action. Applied immediately, between ticks.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Command {
    SetRoutine {
        agent: String,
        slot: String,
        activity: Option<String>,
    },
    PlaceStructure {
        x: i32,
        y: i32,
        structure: String,
    },
    RemoveStructure {
        x: i32,
        y: i32,
    },
    AdjustPatch {
        x: i32,
        y: i32,
        stock: String,
        delta: i32,
    },
    /// Adds to an agent's need for the next tick only.
    AdjustNeed {
        agent: String,
        need: String,
        delta: i32,
    },
    /// Fixes the next tick's rain (a disturbance or a gift).
    SetRain {
        amount: i32,
    },
}

/// One value that changed, with before and after.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Change {
    /// `patch` or `agent`.
    pub scope: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub x: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub y: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub agent: Option<String>,
    pub what: String,
    pub from: i32,
    pub to: i32,
}

/// Something that happened, with every value it changed.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Event {
    pub tick: u32,
    /// `commands`, `weather`, `activities`, `structures`, `growth`, `needs`, `species`.
    pub phase: String,
    pub kind: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub subject: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub detail: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub value: Option<i32>,
    pub changes: Vec<Change>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Rejection {
    pub index: usize,
    pub reason: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ApplyResult {
    pub events: Vec<Event>,
    pub rejected: Vec<Rejection>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Snapshot {
    pub engine_version: String,
    pub pack_id: String,
    pub pack_version: String,
    pub world: World,
}
