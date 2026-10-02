use understory_core::*;

fn pack() -> Pack {
    serde_json::from_str(include_str!("../../../packs/reference/pack.json")).unwrap()
}
fn world() -> WorldDef {
    serde_json::from_str(include_str!("../../../packs/reference/world.json")).unwrap()
}
fn script() -> Vec<(u32, Command)> {
    vec![
        (
            0,
            Command::SetRoutine {
                agent: "ana".into(),
                slot: "home".into(),
                activity: Some("balcony_garden".into()),
            },
        ),
        (
            0,
            Command::SetRoutine {
                agent: "bo".into(),
                slot: "home".into(),
                activity: Some("compost".into()),
            },
        ),
        (
            2,
            Command::SetRoutine {
                agent: "cy".into(),
                slot: "commons".into(),
                activity: Some("shared_meal".into()),
            },
        ),
        (
            3,
            Command::PlaceStructure {
                x: 1,
                y: 1,
                structure: "community_garden".into(),
            },
        ),
        (5, Command::SetRain { amount: 3 }),
    ]
}
fn run(seed: u32, ticks: u32) -> Engine {
    let mut e = Engine::new(pack(), world(), seed).unwrap();
    for t in 0..ticks {
        let cmds: Vec<Command> = script()
            .into_iter()
            .filter(|(at, _)| *at == t)
            .map(|(_, c)| c)
            .collect();
        let r = e.apply(&cmds);
        assert!(r.rejected.is_empty(), "{:?}", r.rejected);
        e.step();
    }
    e
}

#[test]
fn same_inputs_same_hash() {
    assert_eq!(run(42, 12).hash(), run(42, 12).hash());
}

#[test]
fn different_seed_differs() {
    // Weather variance makes at least one of these seeds diverge.
    let h = run(1, 12).hash();
    assert!((2..12).any(|s| run(s, 12).hash() != h));
}

/// Golden hash. If this changes, saves and replays made with earlier
/// versions are invalid: treat it as a breaking change and say so in the
/// changelog.
#[test]
fn golden_hash() {
    assert_eq!(run(42, 12).hash(), "da24e4417bf845b2");
}

#[test]
fn stocks_and_needs_stay_in_bounds() {
    let p = pack();
    for seed in 0..50 {
        let e = run(seed, 20);
        for patch in &e.world().patches {
            let t = &p.terrains[&patch.terrain];
            assert!((0..=p.stock_max).contains(&patch.soil));
            assert!((0..=p.stock_max).contains(&patch.water));
            assert!(patch.biomass >= 0 && patch.biomass <= t.max_biomass);
            let cap = t.water_capacity + if t.soil_adds_capacity { patch.soil } else { 0 };
            assert!(patch.water <= cap.min(p.stock_max));
        }
        for a in &e.world().agents {
            for v in a.needs.values() {
                assert!((0..=p.need_max).contains(v));
            }
        }
    }
}

#[test]
fn every_change_is_reported() {
    // Replaying the reported changes over the old state must give the new state.
    let mut e = Engine::new(pack(), world(), 9).unwrap();
    e.apply(&[Command::SetRoutine {
        agent: "ana".into(),
        slot: "home".into(),
        activity: Some("balcony_garden".into()),
    }]);
    for _ in 0..8 {
        let before = e.world().clone();
        let events = e.step();
        let mut patches = before.patches.clone();
        for ev in events.iter().filter(|ev| ev.phase != "needs") {
            for c in ev.changes.iter().filter(|c| c.scope == "patch") {
                let p = patches
                    .iter_mut()
                    .find(|p| Some(p.x) == c.x && Some(p.y) == c.y)
                    .unwrap();
                let cur = match c.what.as_str() {
                    "soil" => &mut p.soil,
                    "water" => &mut p.water,
                    _ => &mut p.biomass,
                };
                assert_eq!(*cur, c.from);
                *cur = c.to;
            }
        }
        assert_eq!(patches, e.world().patches);
    }
}

#[test]
fn growth_needs_soil_and_water() {
    let mut e = Engine::new(pack(), world(), 3).unwrap();
    e.apply(&[Command::SetRain { amount: 2 }]);
    e.step();
    // lot at (1,0) has soil 1: it grows to 1. lot at (2,2) has soil 0: it stays bare.
    let w = e.world();
    assert_eq!(w.patches[1].biomass, 1);
    assert_eq!(w.patches[10].biomass, 0);
}

#[test]
fn invalid_commands_are_rejected_and_change_nothing() {
    let mut e = Engine::new(pack(), world(), 1).unwrap();
    let h = e.hash();
    let r = e.apply(&[
        Command::SetRoutine {
            agent: "nobody".into(),
            slot: "home".into(),
            activity: None,
        },
        Command::SetRoutine {
            agent: "ana".into(),
            slot: "work".into(),
            activity: Some("compost".into()),
        },
        Command::PlaceStructure {
            x: 0,
            y: 0,
            structure: "community_garden".into(),
        },
        Command::PlaceStructure {
            x: 9,
            y: 9,
            structure: "community_garden".into(),
        },
    ]);
    assert_eq!(r.rejected.len(), 4);
    assert_eq!(e.hash(), h);
}

#[test]
fn snapshot_round_trip_continues_identically() {
    let mut a = run(5, 6);
    let json = serde_json::to_string(&a.snapshot()).unwrap();
    let mut b = Engine::restore(pack(), serde_json::from_str(&json).unwrap()).unwrap();
    for _ in 0..6 {
        a.step();
        b.step();
    }
    assert_eq!(a.hash(), b.hash());
}

#[test]
fn species_arrive_with_habitat() {
    let e = run(42, 12);
    assert!(e.world().species.contains("bee"));
}

#[test]
fn bad_pack_is_refused() {
    let mut p = pack();
    p.activities.get_mut("compost").unwrap().slot = "nowhere".into();
    assert!(Engine::new(p, world(), 1).is_err());
}
