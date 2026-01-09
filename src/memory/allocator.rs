use std::cmp::Ordering;

#[derive(Clone, Debug)]
pub struct Block {
    pub start: usize,
    pub size: usize,
    pub free: bool,
    pub id: Option<u32>,
}

#[derive(Clone, Copy)]
pub enum AllocationStrategy {
    FirstFit,
    BestFit,
    WorstFit,
}

pub struct MemoryAllocator {
    pub total: usize,
    pub blocks: Vec<Block>,
    pub strategy: AllocationStrategy,
    pub next_id: u32,

    pub alloc_attempts: usize,
    pub alloc_success: usize,
}

impl MemoryAllocator {
    pub fn new(size: usize) -> Self {
        Self {
            total: size,
            blocks: vec![Block { start: 0, size, free: true, id: None }],
            strategy: AllocationStrategy::FirstFit,
            next_id: 1,
            alloc_attempts: 0,
            alloc_success: 0,
        }
    }

    pub fn set_strategy(&mut self, s: AllocationStrategy) {
        self.strategy = s;
    }

    pub fn malloc(&mut self, size: usize) -> Option<(u32, usize)> {
        self.alloc_attempts += 1;

        let mut candidate: Option<usize> = None;

        for (i, b) in self.blocks.iter().enumerate() {
            if b.free && b.size >= size {
                match self.strategy {
                    AllocationStrategy::FirstFit => {
                        candidate = Some(i);
                        break;
                    }
                    AllocationStrategy::BestFit => {
                        if let Some(ci) = candidate {
                            if b.size < self.blocks[ci].size {
                                candidate = Some(i);
                            }
                        } else {
                            candidate = Some(i);
                        }
                    }
                    AllocationStrategy::WorstFit => {
                        if let Some(ci) = candidate {
                            if b.size > self.blocks[ci].size {
                                candidate = Some(i);
                            }
                        } else {
                            candidate = Some(i);
                        }
                    }
                }
            }
        }

        let i = candidate?;

        let (start, bsize) = (self.blocks[i].start, self.blocks[i].size);

        // allocate ID
        let id = self.next_id;
        self.next_id += 1;

        if bsize == size {
            // exact fit
            self.blocks[i].free = false;
            self.blocks[i].id = Some(id);
        } else {
            // split
            let new_block = Block {
                start,
                size,
                free: false,
                id: Some(id),
            };

            let remainder = Block {
                start: start + size,
                size: bsize - size,
                free: true,
                id: None,
            };

            self.blocks.remove(i);
            self.blocks.insert(i, new_block);
            self.blocks.insert(i + 1, remainder);
        }

        self.alloc_success += 1;
        Some((id, start))
    }

    pub fn free(&mut self, id: u32) -> bool {
        let mut index = None;
        for (i, b) in self.blocks.iter().enumerate() {
            if !b.free && b.id == Some(id) {
                index = Some(i);
                break;
            }
        }
        let i = match index { Some(v) => v, None => return false };

        self.blocks[i].free = true;
        self.blocks[i].id = None;

        self.coalesce();
        true
    }

    fn coalesce(&mut self) {
        let mut i = 0;
        while i + 1 < self.blocks.len() {
            if self.blocks[i].free && self.blocks[i + 1].free {
                let sz = self.blocks[i].size + self.blocks[i + 1].size;
                self.blocks[i].size = sz;
                self.blocks.remove(i + 1);
            } else {
                i += 1;
            }
        }
    }

    pub fn stats(&self) {
        let used: usize = self.blocks.iter().filter(|b| !b.free).map(|b| b.size).sum();
        let free: usize = self.total - used;

        // external fragmentation
        let mut max_free = 0;
        for b in self.blocks.iter().filter(|b| b.free) {
            max_free = max_free.max(b.size);
        }
        let ext = if free > 0 {
            (1.0 - (max_free as f64 / free as f64)) * 100.0
        } else {
            0.0
        };

        let rate = if self.alloc_attempts > 0 {
            (self.alloc_success as f64 / self.alloc_attempts as f64) * 100.0
        } else {
            100.0
        };

        println!("--- Memory Stats ---");
        println!("Total memory: {} bytes", self.total);
        println!("Used memory: {} bytes", used);
        println!("Free memory: {} bytes", free);
        println!("Utilization: {:.2}%", (used as f64 / self.total as f64) * 100.0);
        println!("External fragmentation: {:.2}%", ext);
        println!("Internal fragmentation: 0.00%");
        println!("Allocation success rate: {:.2}%", rate);
    }

    pub fn dump(&self) {
        println!("--- Memory Layout ---");
        for b in &self.blocks {
            if b.free {
                println!("[FREE start={} size={}]", b.start, b.size);
            } else {
                println!("[ALLOC id={} start={} size={}]", b.id.unwrap(), b.start, b.size);
            }
        }
    }
}
