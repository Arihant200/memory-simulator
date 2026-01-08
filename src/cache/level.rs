use super::{Cache, ReplacementPolicy};

pub struct CacheLevel {
    pub l1: Cache,
    pub l2: Cache,
}

impl CacheLevel {
    pub fn new(l1_size: usize, l2_size: usize, block_size: usize, assoc: usize, policy: ReplacementPolicy) -> Self {
        Self {
            l1: Cache::new(l1_size, block_size, assoc, policy),
            l2: Cache::new(l2_size, block_size, assoc, policy),
        }
    }

    pub fn access(&mut self, addr: u64) -> bool {
        if self.l1.access(addr) {
            return true;
        }
        if self.l2.access(addr) {
            // promotion could be added here in later phase
            return false;
        }
        false
    }

    pub fn stats(&self) {
        println!("--- L1 Stats ---");
        self.l1.stats();
        println!("--- L2 Stats ---");
        self.l2.stats();
    }
}
