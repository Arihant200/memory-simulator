#[derive(Clone)]
pub struct Block {
    pub id: Option<u32>,
    pub start: usize,
    pub size: usize,
    pub allocated: bool,
}

impl Block {
    pub fn new_free(start: usize, size: usize) -> Self {
        Block {
            id: None,
            start,
            size,
            allocated: false,
        }
    }

    pub fn new_alloc(start: usize, size: usize, id: u32) -> Self {
        Block {
            id: Some(id),
            start,
            size,
            allocated: true,
        }
    }
}
