use axum::{routing::get, Router, response::Html};
use orderbook::order::{Order, Side};
use orderbook::book::OrderBook;
use std::sync::{Arc, Mutex};
use std::time::Instant;
use tokio::sync::broadcast;
use serde::Serialize;

#[derive(Clone, Serialize)]
struct BenchStats {
    orders_per_sec:    f64,
    avg_latency_ns:    f64,
    p99_latency_ns:    f64,
    total_orders:      u64,
    total_fills:       u64,
    best_bid:          f64,
    best_ask:          f64,
    spread:            f64,
    throughput_label:  String,
}


#[tokio::main]
async fn main() {
    // ── Check correctness on boot ──────────────────────────────────────────
    let mut startup_book = OrderBook::new();
    startup_book.process(Order::limit(1, Side::Buy,  10005, 200));
    startup_book.process(Order::limit(2, Side::Buy,  10000, 500));
    startup_book.process(Order::limit(3, Side::Sell, 10100, 150));
    startup_book.process(Order::limit(4, Side::Sell, 10115, 800));
    
    println!("--- Matching Engine Boot Sequence Active ---");
    println!("Initial Best Bid: ${:.2}", startup_book.best_bid().unwrap_or(0) as f64 / 100.0);
    println!("Initial Best Ask: ${:.2}", startup_book.best_ask().unwrap_or(0) as f64 / 100.0);
    println!("Initial Spread:   ${:.2}", startup_book.spread().unwrap_or(0) as f64 / 100.0);

    // ── Global telemetry registration state ────────────────────────────────
    let stats = Arc::new(Mutex::new(BenchStats {
        orders_per_sec:   0.0, avg_latency_ns: 0.0, p99_latency_ns: 0.0,
        total_orders:     0,   total_fills:    0,
        best_bid:         0.0, best_ask:       0.0, spread: 0.0,
        throughput_label: "warming up...".into(),
    }));

    let (tx, _) = broadcast::channel::<String>(128);

    // ── Continuous automated trade engine matching worker loop ─────────────
    {
        let stats = stats.clone();
        let tx = tx.clone();
        tokio::spawn(async move {
            let mut total_orders: u64 = 0;
            let mut total_fills:  u64 = 0;
            let mut id: u64 = 100;
            let window = 50_000u64;

            loop {
                let mut book = OrderBook::new();
                let mut latencies: Vec<u64> = Vec::with_capacity(window as usize);

                // Seed some liquidity buffers across our price map bounds
                for i in 0..200u64 {
                    id += 1;
                    book.process(Order::limit(id, Side::Buy,  9900 + i, 100));
                    id += 1;
                    book.process(Order::limit(id, Side::Sell, 10100 + i, 100));
                }

                let batch_start = Instant::now();
                let mut batch_fills: u64 = 0;

                for i in 0..window {
                    id += 1;
                    let side = if i % 2 == 0 { Side::Buy } else { Side::Sell };
                    let price = if matches!(side, Side::Buy) { 10000 + (i % 5) } else { 9995 + (i % 5) };

                    let t0 = Instant::now();
                    let fills = book.process(Order::limit(id, side, price, 50));
                    let elapsed = t0.elapsed().as_nanos() as u64;

                    latencies.push(elapsed);
                    batch_fills += fills.len() as u64;
                }

                let elapsed_secs = batch_start.elapsed().as_secs_f64();
                total_orders += window;
                total_fills  += batch_fills;

                latencies.sort_unstable();
                let avg = latencies.iter().sum::<u64>() as f64 / latencies.len() as f64;
                let p99 = latencies[(latencies.len() as f64 * 0.99) as usize] as f64;
                let ops = window as f64 / elapsed_secs;

                let label = if ops >= 1_000_000.0 {
                    format!("{:.2}M orders/sec", ops / 1_000_000.0)
                } else {
                    format!("{:.0}K orders/sec", ops / 1_000.0)
                };

                let bid  = book.best_bid().unwrap_or(0) as f64 / 100.0;
                let ask  = book.best_ask().unwrap_or(0) as f64 / 100.0;
                let sprd = book.spread().unwrap_or(0) as f64 / 100.0;

                let s = BenchStats {
                    orders_per_sec:   ops,
                    avg_latency_ns:   avg,
                    p99_latency_ns:   p99,
                    total_orders,
                    total_fills,
                    best_bid:         bid,
                    best_ask:         ask,
                    spread:           sprd,
                    throughput_label: label,
                };

                if let Ok(json) = serde_json::to_string(&s) {
                    let _ = tx.send(json);
                }
                *stats.lock().unwrap() = s;

                tokio::time::sleep(tokio::time::Duration::from_millis(800)).await;
            }
        });
    }

    // ── Web Interface Core Server Engine Routing ───────────────────────────
    let app = Router::new()
        .route("/", get(serve_dashboard))
        .route("/ws", get({
            let tx = tx.clone();
            move |ws: axum::extract::WebSocketUpgrade| {
                let tx = tx.clone();
                async move {
                    ws.on_upgrade(move |mut socket| async move {
                        let mut rx = tx.subscribe();
                        while let Ok(msg) = rx.recv().await {
                            if socket.send(axum::extract::ws::Message::Text(msg.into())).await.is_err() {
                                break;
                            }
                        }
                    })
                }
            }
        }));

    println!("\n🚀 Live Architecture Operational. Monitoring System Connected.");
    println!("Dashboard URL → http://localhost:3000\n");
    
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn serve_dashboard() -> Html<&'static str> {
    Html(include_str!("dashboard.html"))
}