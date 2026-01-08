mod paging;
mod page_table;
mod replacement;

pub use paging::{VirtualMemory, VMState};
pub use replacement::PageReplacementPolicy;

