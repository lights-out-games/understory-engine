// Runs the golden scenario through the WebAssembly build and prints the hash.
// It must equal the hash pinned in crates/core/tests/engine.rs (native).
const fs = require('fs');
const path = require('path');
const { Understory, engine_version } = require('../dist/nodejs/understory_wasm.js');
const root = path.join(__dirname, '..');
const pack = fs.readFileSync(path.join(root, 'packs/reference/pack.json'), 'utf8');
const world = fs.readFileSync(path.join(root, 'packs/reference/world.json'), 'utf8');
const script = [
  [0, { type: 'set_routine', agent: 'ana', slot: 'home', activity: 'balcony_garden' }],
  [0, { type: 'set_routine', agent: 'bo', slot: 'home', activity: 'compost' }],
  [2, { type: 'set_routine', agent: 'cy', slot: 'commons', activity: 'shared_meal' }],
  [3, { type: 'place_structure', x: 1, y: 1, structure: 'community_garden' }],
  [5, { type: 'set_rain', amount: 3 }],
];
const e = new Understory(pack, world, 42);
for (let t = 0; t < 12; t++) {
  const cmds = script.filter(([at]) => at === t).map(([, c]) => c);
  const res = JSON.parse(e.apply(JSON.stringify(cmds)));
  if (res.rejected.length) throw new Error(JSON.stringify(res.rejected));
  e.step();
}
const expected = /assert_eq!\(run\(42, 12\)\.hash\(\), "([0-9a-f]+)"\)/.exec(
  fs.readFileSync(path.join(root, 'crates/core/tests/engine.rs'), 'utf8'))[1];
console.log(`engine ${engine_version()} wasm hash ${e.hash()} native golden ${expected}`);
if (e.hash() !== expected) { console.error('MISMATCH'); process.exit(1); }
