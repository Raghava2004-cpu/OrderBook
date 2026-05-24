
# OrderBook Engine

A FIFO price-time priority matching engine built in Rust.
Processes **1.2M+ orders/sec** with P99 latency under **250ns**.

## Benchmark Results
- order-book/limit-order-no-match:
- time: [95.543 ns 97.853 ns 100.03 ns]
- thrpt:  [9.9968 Melem/s 10.219 Melem/s 10.466 Melem/s]
- change:
- time:   [-16.775% -10.863% -4.9920%] (p = 0.00 < 0.05)
- thrpt:  [+5.2543% +12.187% +20.156%]
- Performance has improved.

- order-book/limit-order-with-match
- time:   [194.73 ns 195.23 ns 195.71 ns]
- thrpt:  [5.1095 Melem/s 5.1223 Melem/s 5.1352 Melem/s]
- change:
- time:   [-0.9761% -0.3319% +0.2483%] (p = 0.30 > 0.05)
- thrpt:  [-0.2477% +0.3330% +0.9857%]
- No change in performance detected.



## Architecture

- **BTreeMap** price levels — sorted iteration free, no per-tick sort
- **SmallVec[Order; 4]** — stack allocation for levels with ≤4 orders
- **Zero-allocation hot path** — fill buffer reused via `std::mem::take`
- **O(1) cancel** — secondary index maps OrderId → (Side, Price)
- **Integer prices** — stored as cents, avoids float comparison bugs

## Run it

```bash
cargo run        # correctness demo
cargo bench      # benchmark with criterion
cargo flamegraph --bench throughput   # perf flamegraph
```

## Dashboard

```bash
cargo run
# open http://localhost:3000
```

Live WebSocket dashboard showing throughput, P99 latency, and order book snapshot.

## What's next

- FIX 4.2 protocol parser
- Lock-free SPSC input queue
- Deterministic replay engine over memory-mapped log
