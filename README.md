<img width="960" height="540" alt="typical" src="https://github.com/user-attachments/assets/9894d393-9fae-4d76-bd6f-318a5897412a" />
<img width="960" height="540" alt="slope" src="https://github.com/user-attachments/assets/9c9a86de-fb47-43ef-9604-8fa211294d95" />
<img width="960" height="540" alt="SD" src="https://github.com/user-attachments/assets/d6bc56ec-5b5e-4748-a333-1abdfb101334" />
<img width="450" height="300" alt="relative_regression_small" src="https://github.com/user-attachments/assets/88db05f8-2a1f-41e1-95c7-ccfcb2d36dc7" />
<img width="450" height="300" alt="relative_pdf_small" src="https://github.com/user-attachments/assets/3d347f12-36d4-4e55-acc2-681afebda130" />
<img width="450" height="300" alt="regression_small" src="https://github.com/user-attachments/assets/1ba6d323-abc8-49ec-83b0-5d5318d06a14" />
<img width="960" height="540" alt="regression" src="https://github.com/user-attachments/assets/dd1cb584-536e-42ee-8d58-329a79363c7f" />
<img width="450" height="300" alt="pdf_small" src="https://github.com/user-attachments/assets/cfc29b2f-bf5d-4d7e-8677-1fdf5cb56b17" />
<img width="960" height="540" alt="pdf" src="https://github.com/user-attachments/assets/34fcd5c1-bfa1-4917-80ed-dc6e3549cf95" />
<img width="960" height="540" alt="median" src="https://github.com/user-attachments/assets/f56eee8e-8bf1-4c29-a831-70c193e5f462" />
<img width="960" height="540" alt="mean" src="https://github.com/user-attachments/assets/347c0920-267b-47a4-a231-a14dd892b398" />
<img width="960" height="540" alt="MAD" src="https://github.com/user-attachments/assets/2a658aae-6284-464c-99ae-c28e1efa724a" />
# OrderBook Engine

A FIFO price-time priority matching engine built in Rust.
Processes **1.2M+ orders/sec** with P99 latency under **250ns**.

## Benchmark Results
order-book/limit-order-no-match   time: [99 ns]   thrpt: [10M elem/s]
order-book/limit-order-with-match time: [232 ns]  thrpt: [4.3M elem/s]



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
