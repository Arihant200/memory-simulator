# Memory Management Simulator

A user-space memory system simulator implementing physical memory allocators, virtual memory with paging, and CPU cache models. Designed as a learning + systems exploration tool with a CLI interface.

---




## 🚀 Features

### **Physical Memory Allocation**
- First Fit / Best Fit / Worst Fit
- Block splitting and coalescing
- Fragmentation metrics:
  - External fragmentation
  - (Internal fragmentation for paging)
- Allocation statistics (success/fail rate, utilization)

### **Virtual Memory (Phase 4)**
- 32-bit virtual address space
- 4 KB page size
- Virtual → Physical address translation
- Per-process page table simulation
- Page faults + statistics
- LRU page replacement
- Frame allocation (8-frame physical memory)

### **Multilevel Cache Simulation**
- L1 + L2 caches
- 4-way set associative
- 64-byte block size
- LRU replacement
- Hit/Miss counters + statistics
- Integrated VM → Cache → Memory pipeline

---


## 🧠 Architecture Overview

```
+-----------+       +---------+       +---------+       +---------+
| CPU (VA)  |  -->  |  VM MMU |  -->  |  Cache  |  -->  | Memory  |
+-----------+       +---------+       +---------+       +---------+
   |                   |                   |                 |
   |                   |                   |                 |
   |         Translate VA→PA        L1/L2 lookup      Frame access
   |                   |                   |                 |
   |         Page Fault Handling           |                 |
   |         Page Replacement (LRU)        |                 |
```



## 📦 CLI Usage

Start simulator:

cargo run

### **Memory Commands**
```
init <size>
set_allocator <first|best|worst>
malloc <bytes>
free <id>
dump
stats
```



### **VM Commands**
```
vm_init
vm_access <address>
vm_stats
```



### **Cache Commands**
```
cache_init <l1_size> <l2_size>
cache_access <addr>
cache_stats
```



---

## 🧪 Example Session

```
> init 1024
> set_allocator first
> malloc 200
> malloc 50
> free 1
> stats
```

VM + Cache:

```
> cache_init 32768 262144
> vm_init
> vm_access 0x1000
> vm_access 0x1000
> vm_stats
> cache_stats
```

Expected output:

```
VM Stats:
Page Faults: 1

Cache:
Accesses: 2
Hits: 1
Misses: 1
Hit Rate: 50.00%
```

---

## 🎥 Demo Video



<p align="center">
  <a href="https://drive.google.com/file/d/1MDlsJzO9CH_MISBmZOf3ihJ4jcxXkXjI/view?usp=sharing">
    <img src="https://img.shields.io/badge/Watch%20Demo%20Video-Google%20Drive-blue?logo=google-drive&style=for-the-badge" />
  </a>
</p>

> Demonstrates the correctness, internal dynamics, and performance characteristics of the simulator on real workloads.



## 📊 Metrics & Observability

| Component | Metrics |
|---|---|
| Allocator | fragmentation, utilization, alloc success rate |
| VM | page faults, replacement events |
| Cache | hit/miss, hit-rate, level statistics |

---

## 🧩 Internals & Algorithms

| Subsystem | Algorithm |
|---|---|
| Allocator | First Fit / Best Fit / Worst Fit |
| VM Replacement | LRU |
| Cache Replacement | LRU |
| Cache Organization | 4-way set associative |
| Page Size | 4 KB |
| Address Space | 32-bit |

---

## 📚 What I Learned

- How virtual → physical address translation works
- How page tables, frames, and page faults interact
- How LRU page replacement policies are implemented
- How L1/L2 caches affect memory performance
- Internal & external fragmentation in allocators
- Systems-level pipeline thinking (VM → Cache → Memory)

---

## 🛠 Build & Run

Prerequisites: Rust toolchain

cargo run


---


## Testing

The simulator includes predefined workloads located in `/tests`.



Run all workloads and generate logs:

    ./tests/run_tests.ps1

Logs are written to `/logs` and include:

- alloc.log    (allocation traces + stats)
- vm.log       (virtual memory traces + page faults)
- cache.log    (cache access traces + hit/miss stats)

Correctness expectations are documented in `tests/CORRECTNESS.md`.


## 🧱 Tech Stack

- Rust
- CLI
- Systems Programming
- OS Virtual Memory Concepts

---


## 📄 License

MIT License



