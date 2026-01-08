#[derive(Clone)]
pub struct PageTableEntry {
    pub valid: bool,
    pub frame: usize,
    pub last_used: u64,
}

pub struct PageTable {
    pub entries: Vec<PageTableEntry>,
}

impl PageTable {
    pub fn new(num_pages: usize) -> Self {
        let mut entries = Vec::new();
        for _ in 0..num_pages {
            entries.push(PageTableEntry {
                valid: false,
                frame: 0,
                last_used: 0,
            });
        }
        Self { entries }
    }
}
