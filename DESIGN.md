# Memory Management Simulator — Design Document

## 1. Introduction

This simulator models a simplified memory system including:

- physical memory allocators (First Fit, Best Fit, Worst Fit)
- virtual memory with paging and page faults
- a two-level CPU cache (L1/L2) with LRU replacement

The goal is to demonstrate how allocation, paging, and caching interact in modern computer systems and to expose performance-related statistics such as fragmentation, page faults, and cache hit/miss rates.

This is an educational simulator, not a hardware-accurate timing model.

---

## 2. Overall Architecture

At a high level, virtual addresses flow through paging, then cache, then physical memory:

```
CPU (VA)
   │
   ▼
+------------------+
| Virtual Memory   | (Paging / Page Table / LRU)
+------------------+
         │
         ▼
+------------------+
|   CPU Cache      | (L1 → L2 → Memory, LRU)
+------------------+
         │
         ▼
+------------------+
| Physical Memory  | (Frames / Allocator)
+------------------+
```

---

## 3. Memory Layout & Assumptions

The simulator assumes the following:

- **Virtual Address Space:** 32-bit
- **Page Size:** 4 KB (\(2^{12}\))
- **Virtual Pages:** \( \frac{2^{32}}{4096} = 1,048,576 \)
- **Physical Frames:** 8 frames, each 4 KB
- **Allocator Target:** physical memory region
- **Addressability:** byte-level
- **Multi-Process Context:** single process only
- **Protection Bits:** not modeled
- **TLB:** not modeled
- **Swap/Disk:** not modeled (page faults are instantaneous)
- **Endianness:** not relevant to simulation

Memory appears differently to each subsystem:

```
Allocator:     contiguous heap of bytes
VM System:     collection of pages → frames
Cache System:  set-associative indexed blocks
```

---

## 4. Physical Allocation Strategies

The physical memory allocator models classic free-list allocation. Three strategies are implemented:

### 4.1 First Fit (FF)

Selects the first free block that fits the request:

- Fast due to short search
- Tends to increase fragmentation over time

### 4.2 Best Fit (BF)

Selects the smallest big-enough block:

- Reduces external fragmentation
- Costs more search time

### 4.3 Worst Fit (WF)

Selects the largest free block:

- Keeps medium blocks available
- May create large unusable remainders

### 4.4 Splitting & Coalescing

Two structural operations improve space reuse:

```
+-------------------+        +---------+
|  Free Block (X)   | -->    |Alloc|Free|
+-------------------+        +---------+
       Split
```

```
Free Adjacent Blocks (A + B) --> Coalesced into one larger block
```

### 4.5 Fragmentation Tracking

The simulator distinguishes:

- **External Fragmentation:** scattered free blocks prevent allocation
- **Internal Fragmentation:** appears in paging subsystem due to page granularity

---

## 5. Buddy System (Considered, Not Implemented)

While buddy allocation is a popular OS strategy using power-of-two block splits and merges, it is intentionally not implemented. The simulator focuses on free-list allocators to keep the design and CLI interactions simpler.

Buddy allocation may be added as a future extension.

---

## 6. Virtual Memory Model

Virtual memory translates virtual addresses into physical frames using page-based mapping.

### 6.1 Virtual Address Breakdown

Given:
- Page size = 4096 bytes = \(2^{12}\)

Virtual addresses are split as:

```
  VA (32 bits)
+--------------------+-------------+
|      VPN (20b)     | Offset(12b) |
+--------------------+-------------+
```

Where:

- `VPN = VA >> 12`
- `Offset = VA & 0xFFF`

### 6.2 Page Table Structure

Each entry stores:

- `valid` bit
- `frame` index
- `last_used` timestamp (for LRU)

### 6.3 Page Faults

On access to unmapped VPN:

1. Page fault triggered
2. If free frame exists → allocate
3. Else → replace victim using **LRU**

### 6.4 Replacement Policy (LRU)

LRU uses timestamps:

```
Access → entry.last_used = timestamp++
Eviction → smallest last_used wins
```

### 6.5 Simplifications

This model excludes:

- multi-level page tables
- TLB
- permissions (R/W/X)
- dirty/reference bits
- swap/disk latency

---

## 7. Cache Hierarchy & Replacement

The simulator includes a simplified two-level inclusive cache:

- **L1 cache:** 32 KB
- **L2 cache:** 256 KB
- **Associativity:** 4-way
- **Block Size:** 64 bytes
- **Replacement:** LRU
- **Behavior:** read-only

### 7.1 Lookup Sequence

```
CPU request
    │
    ├─> L1 lookup
    │     ├─ hit → return
    │     └─ miss
    │
    ├─> L2 lookup
    │     ├─ hit → refill L1 → return
    │     └─ miss
    │
    └─> memory fetch → refill L2 → refill L1 → return
```

### 7.2 Address Format

Physical addresses are interpreted as:

```
+---------+---------+---------+
|  Tag    | Index   | Offset  |
+---------+---------+---------+
```

- **Offset:** selects position in block
- **Index:** selects set
- **Tag:** compares for match

---

## 8. End-to-End Address Translation Flow

Putting VM and cache together:

```
          VA (virtual address)
                   │
                   ▼
          +--------------------+
          |   Paging (VM)      |
          +--------------------+
                   │ produces
                   ▼
          PA (physical address)
                   │
                   ▼
        +------------------------+
        |     Cache System       |
        |  L1 → L2 → Memory      |
        +------------------------+
                   │
                   ▼
          CPU receives data
```

This demonstrates how:

- VM handles **page faults**
- cache handles **miss/refill**
- allocator provides **physical frames**

---

## 9. Statistics & Observability

The simulator tracks:

| Component | Metrics |
|---|---|
| Allocator | fragmentation %, alloc success rate, utilization |
| VM | page faults, replacement count |
| Cache | hits, misses, hit rate, refill count |

These are emitted through CLI commands such as:

```
stats
vm_stats
cache_stats
```

---

## 10. Limitations & Simplifications

The simulator intentionally does **not** model:

- MMU/TLB hardware timing
- multi-level page tables
- store buffering / writeback cache
- dirty/reference bits
- multi-process scheduling
- disk-backed swap
- buddy allocation
- NUMA
- multicore interactions

These omissions make the design easier to reason about while preserving core functional concepts.

---

## 11. Build & Run Instructions

Requires the Rust toolchain.

**Build:**

```
cargo build --release
```

**Run:**

```
cargo run
```

Example CLI usage:

```
> init 1024
> set_allocator best
> malloc 200
> free 1
> vm_init
> vm_access 0x1000
> cache_stats
```

---

## 12. Conclusion

This simulator demonstrates how fragmentation, page faults, and cache hit/miss behavior emerge from layered memory mechanisms. While simplified, the design reflects real-world OS and architecture concepts and provides useful metrics for evaluating memory system behavior.
