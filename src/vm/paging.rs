const PAGE_SIZE: usize = 4096;

use super::page_table::PageTable;
use super::replacement::PageReplacementPolicy;

pub struct VirtualMemory {
    pub page_size: usize,
    pub num_pages: usize,
    pub offset_bits: u32,
}

impl VirtualMemory {
    pub fn new(va_bits: u32, page_size: usize) -> Self {
        let offset_bits = (page_size as f64).log2() as u32;
        let num_pages = 1 << (va_bits - offset_bits);
        Self {
            page_size,
            num_pages,
            offset_bits,
        }
    }

    pub fn split_address(&self, addr: u32) -> (usize, usize) {
        let offset_mask = (self.page_size - 1) as u32;
        let offset = (addr & offset_mask) as usize;
        let vpn = (addr >> self.offset_bits) as usize;
        (vpn, offset)
    }
}

pub struct VMState {
    pub page_table: PageTable,
    pub frames: Vec<Option<usize>>,
    pub next_frame: usize,
    pub timestamp: u64,

    pub faults: usize,
    pub policy: PageReplacementPolicy,
}

impl VMState {
    pub fn new(num_pages: usize, num_frames: usize, policy: PageReplacementPolicy) -> Self {
        Self {
            page_table: PageTable::new(num_pages),
            frames: vec![None; num_frames],
            next_frame: 0,
            timestamp: 0,
            faults: 0,
            policy,
        }
    }

    pub fn translate(&mut self, vpn: usize, offset: usize) -> Option<usize> {
            self.timestamp += 1;

            // CASE 1: HIT (page already valid)
            if self.page_table.entries[vpn].valid {
                let entry = &mut self.page_table.entries[vpn];
                entry.last_used = self.timestamp;
                return Some(entry.frame * PAGE_SIZE + offset);
            }

            // PAGE FAULT
            self.faults += 1;

            // CASE 2: FREE FRAME EXISTS
            if let Some(idx) = self.frames.iter().position(|f| f.is_none()) {
                self.frames[idx] = Some(vpn);
                let entry = &mut self.page_table.entries[vpn];
                entry.valid = true;
                entry.frame = idx;
                entry.last_used = self.timestamp;
                return Some(idx * PAGE_SIZE + offset);
            }

            // CASE 3: REPLACEMENT NEEDED — pick victim FIRST
            let victim = self.select_victim();

            // update frame -> vpn mapping
            self.frames[victim] = Some(vpn);

            // update page table entry
            let entry = &mut self.page_table.entries[vpn];
            entry.valid = true;
            entry.frame = victim;
            entry.last_used = self.timestamp;

            Some(victim * PAGE_SIZE + offset)
        }


    fn select_victim(&self) -> usize {
    match self.policy {
        PageReplacementPolicy::LRU => {
            let mut oldest = u64::MAX;
            let mut victim = 0;

            for (i, vpn_opt) in self.frames.iter().enumerate() {
                if let Some(vpn) = vpn_opt {
                    let ts = self.page_table.entries[*vpn].last_used;
                    if ts < oldest {
                        oldest = ts;
                        victim = i;
                    }
                }
            }

            victim
        }
    }
}


    pub fn stats(&self) {
        println!("--- VM Stats ---");
        println!("Page faults: {}", self.faults);
    }
}
