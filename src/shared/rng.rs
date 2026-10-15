use std::time::{SystemTime, UNIX_EPOCH};

pub struct Rng(u64);

impl Rng {
    pub fn new() -> Self {
        let seed = SystemTime::now().duration_since(UNIX_EPOCH).map_or(1, |d| d.as_nanos() as u64);
        Self(seed | 1)
    }

    pub fn below(&mut self, n: usize) -> usize {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        (self.0 % n as u64) as usize
    }

    pub fn die(&mut self) -> u8 {
        self.below(6) as u8 + 1
    }
}
