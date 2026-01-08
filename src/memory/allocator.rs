use super::block::Block;
use super::strategies::AllocationStrategy;

pub struct MemoryAllocator {
    total_size: usize,
    blocks: Vec<Block>,
    strategy: AllocationStrategy,
    next_id: u32,
}

impl MemoryAllocator {
    pub fn new(size: usize) -> Self {
        let mut blocks = Vec::new();
        blocks.push(Block::new_free(0, size));
        Self {
            total_size: size,
            blocks,
            strategy: AllocationStrategy::FirstFit,
            next_id: 1,
        }
    }

    pub fn set_strategy(&mut self, s: AllocationStrategy) {
        self.strategy = s;
    }

    pub fn malloc(&mut self, size: usize) -> Option<(u32, usize)> {
        let index_opt = match self.strategy {
            AllocationStrategy::FirstFit => self.find_first(size),
            AllocationStrategy::BestFit  => self.find_best(size),
            AllocationStrategy::WorstFit => self.find_worst(size),
        };

        let idx = index_opt?;
        let free_block = self.blocks[idx].clone();
        if free_block.size < size { return None; }

        let id = self.next_id;
        self.next_id += 1;
        let alloc_start = free_block.start;

        if free_block.size == size {
            self.blocks[idx] = Block::new_alloc(alloc_start, size, id);
        } else {
            self.blocks[idx] = Block::new_alloc(alloc_start, size, id);
            self.blocks.insert(idx + 1, Block::new_free(alloc_start + size, free_block.size - size));
        }

        Some((id, alloc_start))
    }

    pub fn free(&mut self, id: u32) -> bool {
        let mut found = false;
        for block in self.blocks.iter_mut() {
            if block.id == Some(id) {
                block.id = None;
                block.allocated = false;
                found = true;
                break;
            }
        }
        if found {
            self.coalesce();
        }
        found
    }

    fn coalesce(&mut self) {
        let mut i = 0;
        while i + 1 < self.blocks.len() {
            if !self.blocks[i].allocated && !self.blocks[i + 1].allocated {
                let new_size = self.blocks[i].size + self.blocks[i + 1].size;
                self.blocks[i].size = new_size;
                self.blocks.remove(i + 1);
            } else {
                i += 1;
            }
        }
    }

    fn find_first(&self, size: usize) -> Option<usize> {
        self.blocks.iter().position(|b| !b.allocated && b.size >= size)
    }

    fn find_best(&self, size: usize) -> Option<usize> {
        let mut best = None;
        let mut best_size = usize::MAX;
        for (i, b) in self.blocks.iter().enumerate() {
            if !b.allocated && b.size >= size && b.size < best_size {
                best = Some(i);
                best_size = b.size;
            }
        }
        best
    }

    fn find_worst(&self, size: usize) -> Option<usize> {
        let mut best = None;
        let mut best_size = 0;
        for (i, b) in self.blocks.iter().enumerate() {
            if !b.allocated && b.size >= size && b.size > best_size {
                best = Some(i);
                best_size = b.size;
            }
        }
        best
    }

    pub fn dump(&self) {
        for block in &self.blocks {
            let end = block.start + block.size - 1;
            if block.allocated {
                println!("[0x{:04X} - 0x{:04X}] USED (id={})", block.start, end, block.id.unwrap());
            } else {
                println!("[0x{:04X} - 0x{:04X}] FREE", block.start, end);
            }
        }
    }
}
