//! The simulation: validation, commands, and the tick.
use crate::model::*;
use crate::rng::Rng;
use std::collections::{BTreeMap, BTreeSet};

pub const ENGINE_VERSION: &str = env!("CARGO_PKG_VERSION");
const STOCKS: [&str; 3] = ["soil", "water", "biomass"];

pub struct Engine {
    pack: Pack,
    world: World,
}

fn is_stock(s: &str) -> bool {
    STOCKS.contains(&s)
}

/// Checks that every name a pack refers to exists.
pub fn validate_pack(pack: &Pack) -> Result<(), String> {
    if pack.slots.is_empty() {
        return Err("pack.slots is empty".into());
    }
    if pack.terrains.is_empty() {
        return Err("pack.terrains is empty".into());
    }
    let check_effects =
        |owner: &str, effects: &[Effect], is_structure: bool| -> Result<(), String> {
            for e in effects {
                match &e.target {
                    Target::Own => {
                        if is_structure {
                            return Err(format!(
                                "{owner}: target `own` is not valid for a structure"
                            ));
                        }
                        if !pack.needs.contains_key(&e.what) {
                            return Err(format!("{owner}: unknown need `{}`", e.what));
                        }
                    }
                    Target::Neighbours { radius } => {
                        if !pack.needs.contains_key(&e.what) {
                            return Err(format!("{owner}: unknown need `{}`", e.what));
                        }
                        if *radius < 0 {
                            return Err(format!("{owner}: negative radius"));
                        }
                    }
                    Target::Home => {
                        if !is_stock(&e.what) {
                            return Err(format!("{owner}: unknown stock `{}`", e.what));
                        }
                    }
                    Target::Nearest { terrain, radius } => {
                        if !is_stock(&e.what) {
                            return Err(format!("{owner}: unknown stock `{}`", e.what));
                        }
                        if !pack.terrains.contains_key(terrain) {
                            return Err(format!("{owner}: unknown terrain `{terrain}`"));
                        }
                        if *radius < 0 {
                            return Err(format!("{owner}: negative radius"));
                        }
                    }
                    Target::Around { terrain, radius } => {
                        if !is_stock(&e.what) {
                            return Err(format!("{owner}: unknown stock `{}`", e.what));
                        }
                        if let Some(t) = terrain {
                            if !pack.terrains.contains_key(t) {
                                return Err(format!("{owner}: unknown terrain `{t}`"));
                            }
                        }
                        if *radius < 0 {
                            return Err(format!("{owner}: negative radius"));
                        }
                    }
                }
            }
            Ok(())
        };
    for (id, a) in &pack.activities {
        if !pack.slots.contains(&a.slot) {
            return Err(format!("activity `{id}`: unknown slot `{}`", a.slot));
        }
        check_effects(&format!("activity `{id}`"), &a.effects, false)?;
    }
    for (id, s) in &pack.structures {
        for t in &s.terrains {
            if !pack.terrains.contains_key(t) {
                return Err(format!("structure `{id}`: unknown terrain `{t}`"));
            }
        }
        check_effects(&format!("structure `{id}`"), &s.effects, true)?;
    }
    for (id, s) in &pack.species {
        for r in &s.requires {
            if !is_stock(&r.stock) {
                return Err(format!("species `{id}`: unknown stock `{}`", r.stock));
            }
            if let Some(t) = &r.terrain {
                if !pack.terrains.contains_key(t) {
                    return Err(format!("species `{id}`: unknown terrain `{t}`"));
                }
            }
        }
    }
    Ok(())
}

impl Engine {
    /// Builds a world from a content pack, a starting world and a seed.
    pub fn new(pack: Pack, def: WorldDef, seed: u32) -> Result<Engine, String> {
        validate_pack(&pack)?;
        if def.width <= 0 || def.height <= 0 {
            return Err("world width and height must be positive".into());
        }
        if def.patches.len() != (def.width * def.height) as usize {
            return Err(format!(
                "world has {} patches, expected {}",
                def.patches.len(),
                def.width * def.height
            ));
        }
        let mut patches = Vec::with_capacity(def.patches.len());
        for (i, p) in def.patches.iter().enumerate() {
            let t = pack
                .terrains
                .get(&p.terrain)
                .ok_or_else(|| format!("patch {i}: unknown terrain `{}`", p.terrain))?;
            let soil = p.soil.clamp(0, pack.stock_max);
            let mut cap = t.water_capacity;
            if t.soil_adds_capacity {
                cap += soil;
            }
            let cap = cap.clamp(0, pack.stock_max);
            patches.push(Patch {
                x: i as i32 % def.width,
                y: i as i32 / def.width,
                terrain: p.terrain.clone(),
                soil,
                water: p.water.clamp(0, cap),
                biomass: p.biomass.clamp(0, t.max_biomass.min(pack.stock_max)),
                structure: None,
            });
        }
        let mut agents = Vec::new();
        let mut seen = BTreeSet::new();
        for a in &def.agents {
            if a.x < 0 || a.y < 0 || a.x >= def.width || a.y >= def.height {
                return Err(format!("agent `{}` is outside the world", a.id));
            }
            if !seen.insert(a.id.clone()) {
                return Err(format!("duplicate agent id `{}`", a.id));
            }
            agents.push(Agent {
                id: a.id.clone(),
                x: a.x,
                y: a.y,
                needs: pack
                    .needs
                    .iter()
                    .map(|(k, n)| (k.clone(), n.baseline.clamp(0, pack.need_max)))
                    .collect(),
                bonus: BTreeMap::new(),
                slots: pack.slots.iter().map(|s| (s.clone(), None)).collect(),
            });
        }
        let world = World {
            tick: 0,
            seed,
            rng: Rng::new(seed),
            width: def.width,
            height: def.height,
            last_rain: 0,
            rain_override: None,
            patches,
            agents,
            species: BTreeSet::new(),
            stress: BTreeMap::new(),
        };
        Ok(Engine { pack, world })
    }

    pub fn world(&self) -> &World {
        &self.world
    }

    pub fn pack(&self) -> &Pack {
        &self.pack
    }

    /// Full state as ordered JSON.
    pub fn state_json(&self) -> String {
        serde_json::to_string(&self.world).expect("world serialises")
    }

    /// FNV-1a 64-bit hash of the state JSON, as 16 hex digits.
    pub fn hash(&self) -> String {
        let mut h: u64 = 0xcbf29ce484222325;
        for b in self.state_json().as_bytes() {
            h ^= *b as u64;
            h = h.wrapping_mul(0x100000001b3);
        }
        format!("{h:016x}")
    }

    pub fn snapshot(&self) -> Snapshot {
        Snapshot {
            engine_version: ENGINE_VERSION.to_string(),
            pack_id: self.pack.id.clone(),
            pack_version: self.pack.version.clone(),
            world: self.world.clone(),
        }
    }

    /// Restores a snapshot made with the same pack and engine major/minor version.
    pub fn restore(pack: Pack, snap: Snapshot) -> Result<Engine, String> {
        validate_pack(&pack)?;
        if snap.pack_id != pack.id || snap.pack_version != pack.version {
            return Err(format!(
                "snapshot is for pack {}@{}, got {}@{}",
                snap.pack_id, snap.pack_version, pack.id, pack.version
            ));
        }
        let mm = |v: &str| v.split('.').take(2).collect::<Vec<_>>().join(".");
        if mm(&snap.engine_version) != mm(ENGINE_VERSION) {
            return Err(format!(
                "snapshot is from engine {}, this is {}",
                snap.engine_version, ENGINE_VERSION
            ));
        }
        Ok(Engine {
            pack,
            world: snap.world,
        })
    }

    // ---- geometry -------------------------------------------------------

    fn idx(&self, x: i32, y: i32) -> Option<usize> {
        if x < 0 || y < 0 || x >= self.world.width || y >= self.world.height {
            None
        } else {
            Some((y * self.world.width + x) as usize)
        }
    }

    /// Chebyshev distance: diagonals count as adjacent, as on a paper grid.
    fn dist(ax: i32, ay: i32, bx: i32, by: i32) -> i32 {
        (ax - bx).abs().max((ay - by).abs())
    }

    fn patches_around(&self, origin: usize, radius: i32, terrain: Option<&str>) -> Vec<usize> {
        let (ox, oy) = (self.world.patches[origin].x, self.world.patches[origin].y);
        self.world
            .patches
            .iter()
            .enumerate()
            .filter(|(i, p)| {
                *i != origin
                    && Self::dist(ox, oy, p.x, p.y) <= radius
                    && terrain.map_or(true, |t| p.terrain == t)
            })
            .map(|(i, _)| i)
            .collect()
    }

    fn nearest(&self, origin: usize, terrain: &str, radius: i32) -> Option<usize> {
        let (ox, oy) = (self.world.patches[origin].x, self.world.patches[origin].y);
        let mut best: Option<(i32, usize)> = None;
        for (i, p) in self.world.patches.iter().enumerate() {
            if p.terrain != terrain {
                continue;
            }
            let d = Self::dist(ox, oy, p.x, p.y);
            if d > radius {
                continue;
            }
            if best.map_or(true, |(bd, _)| d < bd) {
                best = Some((d, i));
            }
        }
        best.map(|(_, i)| i)
    }

    fn agents_around(&self, origin: usize, radius: i32, exclude: Option<usize>) -> Vec<usize> {
        let (ox, oy) = (self.world.patches[origin].x, self.world.patches[origin].y);
        self.world
            .agents
            .iter()
            .enumerate()
            .filter(|(i, a)| Some(*i) != exclude && Self::dist(ox, oy, a.x, a.y) <= radius)
            .map(|(i, _)| i)
            .collect()
    }

    // ---- mutation helpers (every change is recorded) --------------------

    fn capacity(&self, i: usize) -> i32 {
        let p = &self.world.patches[i];
        let t = &self.pack.terrains[&p.terrain];
        let mut c = t.water_capacity;
        if t.soil_adds_capacity {
            c += p.soil;
        }
        c.clamp(0, self.pack.stock_max)
    }

    fn patch_add(&mut self, i: usize, what: &str, delta: i32, out: &mut Vec<Change>) {
        let max = match what {
            "water" => self.capacity(i),
            "biomass" => {
                let t = &self.pack.terrains[&self.world.patches[i].terrain];
                t.max_biomass.min(self.pack.stock_max)
            }
            _ => self.pack.stock_max,
        };
        let p = &mut self.world.patches[i];
        let (x, y) = (p.x, p.y);
        let cur = match what {
            "soil" => &mut p.soil,
            "water" => &mut p.water,
            _ => &mut p.biomass,
        };
        let new = (*cur + delta).clamp(0, max.max(0));
        if new != *cur {
            out.push(Change {
                scope: "patch".into(),
                x: Some(x),
                y: Some(y),
                agent: None,
                what: what.to_string(),
                from: *cur,
                to: new,
            });
            *cur = new;
        }
    }

    fn need_add(&mut self, a: usize, need: &str, delta: i32, out: &mut Vec<Change>) {
        let max = self.pack.need_max;
        let agent = &mut self.world.agents[a];
        let cur = agent.needs.entry(need.to_string()).or_insert(0);
        let new = (*cur + delta).clamp(0, max);
        if new != *cur {
            out.push(Change {
                scope: "agent".into(),
                x: None,
                y: None,
                agent: Some(agent.id.clone()),
                what: need.to_string(),
                from: *cur,
                to: new,
            });
            *cur = new;
        }
    }

    fn apply_effect(
        &mut self,
        origin: usize,
        agent: Option<usize>,
        e: &Effect,
        out: &mut Vec<Change>,
    ) {
        match &e.target {
            Target::Own => {
                if let Some(a) = agent {
                    self.need_add(a, &e.what, e.delta, out);
                }
            }
            Target::Home => self.patch_add(origin, &e.what, e.delta, out),
            Target::Nearest { terrain, radius } => {
                if let Some(i) = self.nearest(origin, terrain, *radius) {
                    self.patch_add(i, &e.what, e.delta, out);
                }
            }
            Target::Around { radius, terrain } => {
                for i in self.patches_around(origin, *radius, terrain.as_deref()) {
                    self.patch_add(i, &e.what, e.delta, out);
                }
            }
            Target::Neighbours { radius } => {
                for a in self.agents_around(origin, *radius, agent) {
                    self.need_add(a, &e.what, e.delta, out);
                }
            }
        }
    }

    fn event(&self, phase: &str, kind: &str) -> Event {
        Event {
            tick: self.world.tick,
            phase: phase.into(),
            kind: kind.into(),
            subject: None,
            detail: None,
            value: None,
            changes: Vec::new(),
        }
    }

    // ---- commands -------------------------------------------------------

    /// Applies host commands in order. Invalid commands are rejected with a
    /// reason and change nothing; the rest still apply.
    pub fn apply(&mut self, commands: &[Command]) -> ApplyResult {
        let mut events = Vec::new();
        let mut rejected = Vec::new();
        for (index, c) in commands.iter().enumerate() {
            match self.apply_one(c) {
                Ok(ev) => events.push(ev),
                Err(reason) => rejected.push(Rejection { index, reason }),
            }
        }
        ApplyResult { events, rejected }
    }

    fn agent_index(&self, id: &str) -> Result<usize, String> {
        self.world
            .agents
            .iter()
            .position(|a| a.id == id)
            .ok_or_else(|| format!("unknown agent `{id}`"))
    }

    fn apply_one(&mut self, c: &Command) -> Result<Event, String> {
        match c {
            Command::SetRoutine {
                agent,
                slot,
                activity,
            } => {
                let a = self.agent_index(agent)?;
                if !self.pack.slots.contains(slot) {
                    return Err(format!("unknown slot `{slot}`"));
                }
                if let Some(act) = activity {
                    let def = self
                        .pack
                        .activities
                        .get(act)
                        .ok_or_else(|| format!("unknown activity `{act}`"))?;
                    if &def.slot != slot {
                        return Err(format!("activity `{act}` belongs in slot `{}`", def.slot));
                    }
                }
                self.world.agents[a]
                    .slots
                    .insert(slot.clone(), activity.clone());
                let mut ev = self.event("commands", "set_routine");
                ev.subject = Some(agent.clone());
                ev.detail = Some(format!("{slot}={}", activity.as_deref().unwrap_or("-")));
                Ok(ev)
            }
            Command::PlaceStructure { x, y, structure } => {
                let i = self.idx(*x, *y).ok_or("outside the world")?;
                let def = self
                    .pack
                    .structures
                    .get(structure)
                    .ok_or_else(|| format!("unknown structure `{structure}`"))?;
                let p = &self.world.patches[i];
                if p.structure.is_some() {
                    return Err("patch already has a structure".into());
                }
                if !def.terrains.is_empty() && !def.terrains.contains(&p.terrain) {
                    return Err(format!("`{structure}` cannot stand on `{}`", p.terrain));
                }
                self.world.patches[i].structure = Some(structure.clone());
                let mut ev = self.event("commands", "place_structure");
                ev.subject = Some(format!("{x},{y}"));
                ev.detail = Some(structure.clone());
                Ok(ev)
            }
            Command::RemoveStructure { x, y } => {
                let i = self.idx(*x, *y).ok_or("outside the world")?;
                let old = self.world.patches[i]
                    .structure
                    .take()
                    .ok_or("no structure there")?;
                let mut ev = self.event("commands", "remove_structure");
                ev.subject = Some(format!("{x},{y}"));
                ev.detail = Some(old);
                Ok(ev)
            }
            Command::AdjustPatch { x, y, stock, delta } => {
                let i = self.idx(*x, *y).ok_or("outside the world")?;
                if !is_stock(stock) {
                    return Err(format!("unknown stock `{stock}`"));
                }
                let mut ev = self.event("commands", "adjust_patch");
                ev.subject = Some(format!("{x},{y}"));
                let mut changes = Vec::new();
                self.patch_add(i, stock, *delta, &mut changes);
                ev.changes = changes;
                Ok(ev)
            }
            Command::AdjustNeed { agent, need, delta } => {
                let a = self.agent_index(agent)?;
                if !self.pack.needs.contains_key(need) {
                    return Err(format!("unknown need `{need}`"));
                }
                *self.world.agents[a].bonus.entry(need.clone()).or_insert(0) += *delta;
                let mut ev = self.event("commands", "adjust_need");
                ev.subject = Some(agent.clone());
                ev.detail = Some(need.clone());
                ev.value = Some(*delta);
                Ok(ev)
            }
            Command::SetRain { amount } => {
                self.world.rain_override = Some((*amount).max(0));
                let mut ev = self.event("commands", "set_rain");
                ev.value = Some(*amount);
                Ok(ev)
            }
        }
    }

    // ---- the tick -------------------------------------------------------

    /// Advances one tick and returns everything that happened, in order.
    pub fn step(&mut self) -> Vec<Event> {
        let mut events = Vec::new();

        // 1. Weather: evaporation, then rain up to each patch's capacity.
        let rain = match self.world.rain_override.take() {
            Some(r) => r,
            None => {
                let pat = &self.pack.weather.pattern;
                let base = if pat.is_empty() {
                    0
                } else {
                    pat[self.world.tick as usize % pat.len()]
                };
                let sides = self.pack.weather.variance_sides;
                let swing = if sides >= 2 {
                    let roll = self.world.rng.below(sides);
                    if roll == 0 {
                        -1
                    } else if roll == sides - 1 {
                        1
                    } else {
                        0
                    }
                } else {
                    0
                };
                (base + swing).max(0)
            }
        };
        let evap = self.pack.weather.evaporation;
        let mut ev = self.event("weather", "rain");
        ev.value = Some(rain);
        for i in 0..self.world.patches.len() {
            let cap = self.capacity(i);
            let p = &mut self.world.patches[i];
            let new = ((p.water - evap).max(0) + rain).min(cap);
            if new != p.water {
                ev.changes.push(Change {
                    scope: "patch".into(),
                    x: Some(p.x),
                    y: Some(p.y),
                    agent: None,
                    what: "water".into(),
                    from: p.water,
                    to: new,
                });
                p.water = new;
            }
        }
        self.world.last_rain = rain;
        events.push(ev);

        // 2. Needs start again from baseline (plus any one-tick bonus).
        let before: Vec<BTreeMap<String, i32>> =
            self.world.agents.iter().map(|a| a.needs.clone()).collect();
        let max = self.pack.need_max;
        for a in &mut self.world.agents {
            for (name, def) in &self.pack.needs {
                let b = a.bonus.get(name).copied().unwrap_or(0);
                a.needs
                    .insert(name.clone(), (def.baseline + b).clamp(0, max));
            }
            a.bonus.clear();
        }

        // 3. Activities: agents in order, slots in pack order.
        let slots = self.pack.slots.clone();
        for a in 0..self.world.agents.len() {
            let origin = self
                .idx(self.world.agents[a].x, self.world.agents[a].y)
                .expect("agent home validated");
            for slot in &slots {
                let Some(Some(act)) = self.world.agents[a].slots.get(slot).cloned() else {
                    continue;
                };
                let Some(def) = self.pack.activities.get(&act).cloned() else {
                    continue;
                };
                let mut ev = self.event("activities", "activity");
                ev.subject = Some(self.world.agents[a].id.clone());
                ev.detail = Some(act);
                let mut changes = Vec::new();
                for e in &def.effects {
                    self.apply_effect(origin, Some(a), e, &mut changes);
                }
                ev.changes = changes;
                events.push(ev);
            }
        }

        // 4. Structures: patches in index order.
        for i in 0..self.world.patches.len() {
            let Some(s) = self.world.patches[i].structure.clone() else {
                continue;
            };
            let Some(def) = self.pack.structures.get(&s).cloned() else {
                continue;
            };
            let mut ev = self.event("structures", "structure");
            ev.subject = Some(format!(
                "{},{}",
                self.world.patches[i].x, self.world.patches[i].y
            ));
            ev.detail = Some(s);
            let mut changes = Vec::new();
            for e in &def.effects {
                self.apply_effect(i, None, e, &mut changes);
            }
            ev.changes = changes;
            events.push(ev);
        }

        // 5. Growth: plants grow toward the soil level when watered, and wilt when dry.
        for i in 0..self.world.patches.len() {
            let t = self.pack.terrains[&self.world.patches[i].terrain].clone();
            let p = &self.world.patches[i];
            let limit = t.max_biomass.min(p.soil).min(self.pack.stock_max);
            if t.grows && p.water >= 1 && p.biomass < limit {
                let mut ev = self.event("growth", "growth");
                ev.subject = Some(format!("{},{}", p.x, p.y));
                let mut changes = Vec::new();
                self.patch_add(i, "biomass", 1, &mut changes);
                self.patch_add(i, "water", -1, &mut changes);
                ev.changes = changes;
                events.push(ev);
            } else if p.water == 0 && p.biomass >= 2 {
                let mut ev = self.event("growth", "wilt");
                ev.subject = Some(format!("{},{}", p.x, p.y));
                let mut changes = Vec::new();
                self.patch_add(i, "biomass", -1, &mut changes);
                ev.changes = changes;
                events.push(ev);
            }
        }

        // 6. Needs: report the net change since last tick.
        let mut ev = self.event("needs", "needs");
        for (a, old) in self.world.agents.iter().zip(before.iter()) {
            for (name, now) in &a.needs {
                let was = old.get(name).copied().unwrap_or(0);
                if was != *now {
                    ev.changes.push(Change {
                        scope: "agent".into(),
                        x: None,
                        y: None,
                        agent: Some(a.id.clone()),
                        what: name.clone(),
                        from: was,
                        to: *now,
                    });
                }
            }
        }
        events.push(ev);

        // 7. Species arrive when their habitat exists and leave when it does not.
        let present_before = self.world.species.len() as u32;
        let ids: Vec<String> = self.pack.species.keys().cloned().collect();
        for id in ids {
            let def = &self.pack.species[&id];
            let here = self.world.species.contains(&id);
            let others = present_before - if here { 1 } else { 0 };
            let ok = others >= def.min_species
                && def.requires.iter().all(|r| {
                    let n = self
                        .world
                        .patches
                        .iter()
                        .filter(|p| {
                            r.terrain.as_ref().map_or(true, |t| &p.terrain == t)
                                && match r.stock.as_str() {
                                    "soil" => p.soil,
                                    "water" => p.water,
                                    _ => p.biomass,
                                } >= r.min
                        })
                        .count() as u32;
                    n >= r.patches
                });
            let grace = def.grace;
            if ok {
                self.world.stress.remove(&id);
                if !here {
                    self.world.species.insert(id.clone());
                    let mut ev = self.event("species", "species_arrived");
                    ev.subject = Some(id);
                    events.push(ev);
                }
            } else if here {
                let stress = self.world.stress.get(&id).copied().unwrap_or(0) + 1;
                if stress > grace {
                    self.world.species.remove(&id);
                    self.world.stress.remove(&id);
                    let mut ev = self.event("species", "species_left");
                    ev.subject = Some(id);
                    events.push(ev);
                } else {
                    self.world.stress.insert(id.clone(), stress);
                    let mut ev = self.event("species", "species_stressed");
                    ev.subject = Some(id);
                    ev.value = Some(stress as i32);
                    events.push(ev);
                }
            }
        }

        self.world.tick += 1;
        events
    }
}
