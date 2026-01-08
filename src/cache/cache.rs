use super::ReplacementPolicy;

#[derive(Clone)]
pub struct CacheLine {
    pub valid: bool,
    pub tag: u64,
    pub last_used: u64,
    pub insert_order: u64,
}

pub struct CacheSet {
    pub lines: Vec<CacheLine>,
}

pub struct Cache {
    pub size: usize,
    pub block_size: usize,
    pub associativity: usize,
    pub sets: Vec<CacheSet>,
    pub policy: ReplacementPolicy,

    pub accesses: usize,
    pub hits: usize,
    pub timestamp: u64,
}

impl Cache {
    pub fn new(size: usize, block_size: usize, associativity: usize, policy: ReplacementPolicy) -> Self {
        let num_sets = size / (block_size * associativity);
        let mut sets = Vec::new();

        for _ in 0..num_sets {
            let mut lines = Vec::new();
            for _ in 0..associativity {
                lines.push(CacheLine { valid: false, tag: 0, last_used: 0, insert_order: 0 });
            }
            sets.push(CacheSet { lines });
        }

        Cache {
            size,
            block_size,
            associativity,
            sets,
            policy,
            accesses: 0,
            hits: 0,
            timestamp: 0,
        }
    }

    pub fn access(&mut self, address: u64) -> bool {
        self.accesses += 1;
        self.timestamp += 1;

        let block_addr = address / self.block_size as u64;
        let num_sets = self.sets.len() as u64;
        let index = (block_addr % num_sets) as usize;
        let tag = block_addr / num_sets;

        let set = &mut self.sets[index];

        for line in &mut set.lines {
            if line.valid && line.tag == tag {
                self.hits += 1;
                line.last_used = self.timestamp;
                return true;
            }
        }

        // MISS → find victim to replace
        let victim = match self.policy {
            ReplacementPolicy::FIFO => set.lines.iter_mut().min_by_key(|l| l.insert_order).unwrap(),
            ReplacementPolicy::LRU  => set.lines.iter_mut().min_by_key(|l| l.last_used).unwrap(),
        };

        victim.valid = true;
        victim.tag = tag;
        victim.last_used = self.timestamp;
        victim.insert_order = self.timestamp;

        false
    }

    pub fn stats(&self) {
        let miss = self.accesses - self.hits;
        let hit_rate = (self.hits as f64 / self.accesses as f64) * 100.0;

        println!("Cache Stats:");
        println!("Accesses: {}", self.accesses);
        println!("Hits: {}", self.hits);
        println!("Misses: {}", miss);
        println!("Hit Rate: {:.2}%", hit_rate);
    }
}
