//! Understory core (working name): a deterministic, headless ecosystem
//! simulation. It takes a content pack, a starting world, a seed and host
//! commands, and returns state and events. It reads no files, clocks or
//! network, and uses integer arithmetic only, so the same inputs give the
//! same state hash on every platform.
//!
//! ```
//! use understory_core::*;
//! let pack: Pack = serde_json::from_str(include_str!("../../../packs/reference/pack.json")).unwrap();
//! let world: WorldDef = serde_json::from_str(include_str!("../../../packs/reference/world.json")).unwrap();
//! let mut engine = Engine::new(pack, world, 7).unwrap();
//! let events = engine.step();
//! assert!(!events.is_empty());
//! ```
pub mod engine;
pub mod model;
pub mod rng;

pub use engine::{validate_pack, Engine, ENGINE_VERSION};
pub use model::*;
pub use rng::Rng;
