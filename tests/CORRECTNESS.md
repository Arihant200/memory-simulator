# Correctness & Expected Behavior

This document describes correctness criteria for the memory simulator test workloads.

---

## 1. Allocation Workload

File: `alloc.workload`

Correctness expectations:

- Allocator must successfully allocate 200, 50, 300 bytes initially.
- After freeing block 2, a 50-byte free segment must exist.
- Best Fit should allocate 120 bytes into the smallest freeing block.
- `stats` should report:
  - non-zero external fragmentation
  - high utilization
  - successful allocations count == 4
  - free blocks >= 1

---

## 2. Virtual Memory Workload

File: `vm.workload`

Correctness expectations:

- First access to `0x1000` should cause a page fault.
- Access to `0x2000` should cause a separate page fault.
- Re-access to `0x1000` should NOT fault (hit in page table).
- Final access to `0xF000` should fault.
- Total expected page faults: **3**
- `vm_stats` should display faults == 3

---

## 3. Cache Workload

File: `cache.workload`

Correctness expectations:

Given spatial locality:

- First access to `0x1000` → miss
- `0x1004` and `0x1008` map to same block → hits
- `0x2000` likely maps to different block → miss

Expected pattern:

```
Miss, Hit, Miss, Hit
```

Cache stats expected:

- Hits: 2
- Misses: 2
- Hit rate: 50%
