# OrderBook Engine

A FIFO price-time priority matching engine built in Rust.
Processes **1.2M+ orders/sec** with P99 latency under **250ns**.

## Benchmark Results
order-book/limit-order-no-match   time: [99 ns]   thrpt: [10M elem/s]
order-book/limit-order-with-match time: [232 ns]  thrpt: [4.3M elem/s]

![Flamegraph](flamegraph.svg)

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
