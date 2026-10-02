//! PCG32 random generator. Implemented here so the stream never changes
//! underneath us when a dependency is upgraded.
use serde::{Deserialize, Serialize};

const MULT: u64 = 6364136223846793005;
const INC: u64 = 1442695040888963407;

/// State is stored as two 32-bit halves so that JSON hosts (JavaScript)
/// can round-trip a snapshot without losing precision.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Rng {
    pub hi: u32,
    pub lo: u32,
}

impl Rng {
    pub fn new(seed: u32) -> Self {
        let mut r = Rng { hi: 0, lo: 0 };
        r.next_u32();
        let s = r.state().wrapping_add(seed as u64);
        r.set(s);
        r.next_u32();
        r
    }
    fn state(&self) -> u64 {
        ((self.hi as u64) << 32) | self.lo as u64
    }
    fn set(&mut self, s: u64) {
        self.hi = (s >> 32) as u32;
        self.lo = s as u32;
    }
    pub fn next_u32(&mut self) -> u32 {
        let old = self.state();
        self.set(old.wrapping_mul(MULT).wrapping_add(INC));
        let xorshifted = (((old >> 18) ^ old) >> 27) as u32;
        let rot = (old >> 59) as u32;
        xorshifted.rotate_right(rot)
    }
    /// Uniform integer in `0..n` (n > 0). Integer-only, identical everywhere.
    pub fn below(&mut self, n: u32) -> u32 {
        ((self.next_u32() as u64 * n as u64) >> 32) as u32
    }
}
