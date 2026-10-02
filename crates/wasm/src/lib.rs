//! WebAssembly binding. Every call takes and returns JSON strings so that
//! hosts can log, inspect and store exactly what crossed the boundary.
use understory_core::{Command, Engine, Pack, Snapshot, WorldDef, ENGINE_VERSION};
use wasm_bindgen::prelude::*;

fn err<E: std::fmt::Display>(e: E) -> JsError {
    JsError::new(&e.to_string())
}

#[wasm_bindgen]
pub struct Understory {
    inner: Engine,
}

#[wasm_bindgen]
impl Understory {
    /// `pack_json` and `world_json` follow the schemas in `spec/`.
    #[wasm_bindgen(constructor)]
    pub fn new(pack_json: &str, world_json: &str, seed: u32) -> Result<Understory, JsError> {
        let pack: Pack = serde_json::from_str(pack_json).map_err(|e| err(format!("pack: {e}")))?;
        let world: WorldDef =
            serde_json::from_str(world_json).map_err(|e| err(format!("world: {e}")))?;
        Ok(Understory {
            inner: Engine::new(pack, world, seed).map_err(err)?,
        })
    }

    /// Restores a snapshot produced by `snapshot()`.
    pub fn restore(pack_json: &str, snapshot_json: &str) -> Result<Understory, JsError> {
        let pack: Pack = serde_json::from_str(pack_json).map_err(|e| err(format!("pack: {e}")))?;
        let snap: Snapshot =
            serde_json::from_str(snapshot_json).map_err(|e| err(format!("snapshot: {e}")))?;
        Ok(Understory {
            inner: Engine::restore(pack, snap).map_err(err)?,
        })
    }

    /// Takes a JSON array of commands. Returns `{ events, rejected }`.
    pub fn apply(&mut self, commands_json: &str) -> Result<String, JsError> {
        let cmds: Vec<Command> =
            serde_json::from_str(commands_json).map_err(|e| err(format!("commands: {e}")))?;
        serde_json::to_string(&self.inner.apply(&cmds)).map_err(err)
    }

    /// Advances one tick. Returns a JSON array of events.
    pub fn step(&mut self) -> Result<String, JsError> {
        serde_json::to_string(&self.inner.step()).map_err(err)
    }

    /// Full state as JSON.
    pub fn state(&self) -> String {
        self.inner.state_json()
    }

    /// State hash as 16 hex digits.
    pub fn hash(&self) -> String {
        self.inner.hash()
    }

    pub fn snapshot(&self) -> Result<String, JsError> {
        serde_json::to_string(&self.inner.snapshot()).map_err(err)
    }
}

#[wasm_bindgen]
pub fn engine_version() -> String {
    ENGINE_VERSION.to_string()
}
