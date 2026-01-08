use crate::memory::{MemoryAllocator, AllocationStrategy};
use std::io::{self, Write};

pub struct Shell {
    allocator: Option<MemoryAllocator>,
}

impl Shell {
    pub fn new() -> Self {
        Shell { allocator: None }
    }

    pub fn run(&mut self) {
        loop {
            print!("> ");
            io::stdout().flush().unwrap();

            let mut input = String::new();
            if io::stdin().read_line(&mut input).is_err() {
                continue;
            }
            let cmd = input.trim();
            if cmd == "exit" { break; }

            self.handle(cmd);
        }
    }

    fn handle(&mut self, cmd: &str) {
        let parts: Vec<&str> = cmd.split_whitespace().collect();
        if parts.is_empty() { return; }

        match parts[0] {
            "init" => {
                if parts.len() != 2 {
                    println!("Usage: init <size>");
                    return;
                }
                if let Ok(size) = parts[1].parse::<usize>() {
                    self.allocator = Some(MemoryAllocator::new(size));
                    println!("Initialized memory = {} bytes", size);
                }
            }

            "set_allocator" => {
                if let Some(ref mut alloc) = self.allocator {
                    if parts.len() != 2 {
                        println!("Usage: set_allocator <first_fit|best_fit|worst_fit>");
                        return;
                    }
                    match parts[1] {
                        "first_fit" => alloc.set_strategy(AllocationStrategy::FirstFit),
                        "best_fit"  => alloc.set_strategy(AllocationStrategy::BestFit),
                        "worst_fit" => alloc.set_strategy(AllocationStrategy::WorstFit),
                        _ => {
                            println!("Invalid strategy");
                            return;
                        }
                    }
                    println!("Allocator strategy updated.");
                } else {
                    println!("Memory not initialized.");
                }
            }

            "malloc" => {
                if let Some(ref mut alloc) = self.allocator {
                    if parts.len() != 2 {
                        println!("Usage: malloc <size>");
                        return;
                    }
                    if let Ok(size) = parts[1].parse::<usize>() {
                        match alloc.malloc(size) {
                            Some((id, addr)) => {
                                println!("Allocated block id={} at address=0x{:04X}", id, addr);
                            }
                            None => println!("Allocation failed."),
                        }
                    }
                } else {
                    println!("Memory not initialized.");
                }
            }

            "free" => {
                if let Some(ref mut alloc) = self.allocator {
                    if parts.len() != 2 {
                        println!("Usage: free <id>");
                        return;
                    }
                    if let Ok(id) = parts[1].parse::<u32>() {
                        if alloc.free(id) {
                            println!("Block {} freed and merged", id);
                        } else {
                            println!("Invalid block id");
                        }
                    }
                }
            }

            "dump" => {
                if let Some(ref alloc) = self.allocator {
                    alloc.dump();
                } else {
                    println!("Memory not initialized.");
                }
            }

            _ => println!("Unknown command."),
        }
    }
}
