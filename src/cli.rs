use crate::memory::{MemoryAllocator, AllocationStrategy};
use crate::cache::{CacheLevel, ReplacementPolicy};
use crate::vm::{VirtualMemory, VMState, PageReplacementPolicy};
use std::io::{self, Write};

pub struct Shell {
    allocator: Option<MemoryAllocator>,
    cache: Option<CacheLevel>, // Phase 3
    vm: Option<(VirtualMemory, VMState)>,//phase 4
}

impl Shell {
    pub fn new() -> Self {
        Shell {
        allocator: None,
        cache: None,
        vm: None,
    }
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

            "stats" => {
                if let Some(ref alloc) = self.allocator {
                    alloc.stats();
                } else {
                    println!("Memory not initialized.");
                }
            }

            "cache_init" => {
                if parts.len() < 3 {
                    println!("Usage: cache_init <l1_size> <l2_size>");
                    return;
                }

                let l1 = parts[1].parse::<usize>().unwrap();
                let l2 = parts[2].parse::<usize>().unwrap();

                // block = 64, assoc = 4, policy = LRU for now
                self.cache = Some(CacheLevel::new(l1, l2, 64, 4, ReplacementPolicy::LRU));
                println!("Cache initialized: L1={}B L2={}B block=64B assoc=4 policy=LRU", l1, l2);
            }

            "cache_access" => {
                if let Some(ref mut cache) = self.cache {
                    if parts.len() != 2 {
                        println!("Usage: cache_access <address>");
                        return;
                    }

                    let addr = if parts[1].starts_with("0x") {
                        u64::from_str_radix(&parts[1][2..], 16).unwrap()
                    } else {
                        parts[1].parse::<u64>().unwrap()
                    };

                    let hit = cache.access(addr);
                    if hit {
                        println!("L1 HIT");
                    } else {
                        println!("MISS (L1 & L2)");
                    }
                } else {
                    println!("Cache not initialized.");
                }
            }

            "cache_stats" => {
                if let Some(ref cache) = self.cache {
                    cache.stats();
                } else {
                    println!("Cache not initialized.");
                }
            }

            "vm_init" => {
                let va_bits = 32;
                let page_size = 4096; // 4KB
                let frames = 8;       // your choice
                let policy = PageReplacementPolicy::LRU;

                let vm = VirtualMemory::new(va_bits, page_size);
                let vm_state = VMState::new(vm.num_pages, frames, policy);

                self.vm = Some((vm, vm_state));
                println!("VM initialized: {}-bit VA, {}B pages, {} frames, policy=LRU", va_bits, page_size, frames);
            }

            "vm_access" => {
                if let Some((ref vm, ref mut state)) = self.vm {
                    if parts.len() != 2 {
                        println!("Usage: vm_access <addr>");
                        return;
                    }

                    // parse hex or decimal
                    let addr = if parts[1].starts_with("0x") {
                        u32::from_str_radix(&parts[1][2..], 16).unwrap()
                    } else {
                        parts[1].parse::<u32>().unwrap()
                    };

                    let (vpn, offset) = vm.split_address(addr);
                    if let Some(pa) = state.translate(vpn, offset) {
                        println!("VA 0x{:08X} -> PA 0x{:08X}", addr, pa);

                        // AUTO pipeline to cache now
                        if let Some(ref mut cache) = self.cache {
                            let hit = cache.access(pa as u64);
                            if hit {
                                println!("Cache: HIT");
                            } else {
                                println!("Cache: MISS");
                            }
                        } else {
                            println!("(Cache not initialized)");
                        }
                    }
                } else {
                    println!("VM not initialized.");
                }
            }


            "vm_stats" => {
                if let Some((_, ref vm_state)) = self.vm {
                    vm_state.stats();
                } else {
                    println!("VM not initialized.");
                }
            }



            _ => println!("Unknown command."),
        }
    }
}
