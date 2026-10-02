//! understory run --pack P --world W [--seed N] [--ticks T] [--commands C.jsonl] [--state] [--events]
//! understory validate --pack P
//!
//! The commands file has one JSON object per line: {"tick": 0, "command": {...}}.
//! Commands for tick N are applied before tick N runs.
use serde::Deserialize;
use std::{env, fs, process::exit};
use understory_core::*;

#[derive(Deserialize)]
struct Line {
    tick: u32,
    command: Command,
}

fn arg(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1).cloned())
}

fn fail(msg: String) -> ! {
    eprintln!("error: {msg}");
    exit(1)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let cmd = args.get(1).map(String::as_str).unwrap_or("");
    let read = |name: &str| -> String {
        let path = arg(&args, name).unwrap_or_else(|| fail(format!("missing {name}")));
        fs::read_to_string(&path).unwrap_or_else(|e| fail(format!("{path}: {e}")))
    };
    match cmd {
        "validate" => {
            let pack: Pack = serde_json::from_str(&read("--pack"))
                .unwrap_or_else(|e| fail(format!("pack: {e}")));
            validate_pack(&pack).unwrap_or_else(|e| fail(e));
            println!("ok: {}@{}", pack.id, pack.version);
        }
        "run" => {
            let pack: Pack = serde_json::from_str(&read("--pack"))
                .unwrap_or_else(|e| fail(format!("pack: {e}")));
            let world: WorldDef = serde_json::from_str(&read("--world"))
                .unwrap_or_else(|e| fail(format!("world: {e}")));
            let seed: u32 = arg(&args, "--seed")
                .map(|s| s.parse().unwrap_or_else(|_| fail("bad --seed".into())))
                .unwrap_or(1);
            let ticks: u32 = arg(&args, "--ticks")
                .map(|s| s.parse().unwrap_or_else(|_| fail("bad --ticks".into())))
                .unwrap_or(10);
            let lines: Vec<Line> = match arg(&args, "--commands") {
                Some(p) => fs::read_to_string(&p)
                    .unwrap_or_else(|e| fail(format!("{p}: {e}")))
                    .lines()
                    .filter(|l| !l.trim().is_empty())
                    .map(|l| {
                        serde_json::from_str(l).unwrap_or_else(|e| fail(format!("commands: {e}")))
                    })
                    .collect(),
                None => Vec::new(),
            };
            let mut engine = Engine::new(pack, world, seed).unwrap_or_else(|e| fail(e));
            let show_events = args.iter().any(|a| a == "--events");
            for t in 0..ticks {
                let cmds: Vec<Command> = lines
                    .iter()
                    .filter(|l| l.tick == t)
                    .map(|l| l.command.clone())
                    .collect();
                let res = engine.apply(&cmds);
                for r in &res.rejected {
                    eprintln!("tick {t}: command {} rejected: {}", r.index, r.reason);
                }
                let events = engine.step();
                if show_events {
                    for ev in &events {
                        println!("{}", serde_json::to_string(ev).unwrap());
                    }
                }
            }
            if args.iter().any(|a| a == "--state") {
                println!("{}", engine.state_json());
            }
            println!("tick {} hash {}", engine.world().tick, engine.hash());
        }
        _ => {
            eprintln!("usage:\n  understory run --pack P --world W [--seed N] [--ticks T] [--commands C.jsonl] [--state] [--events]\n  understory validate --pack P");
            exit(2);
        }
    }
}
