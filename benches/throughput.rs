use criterion::{black_box, criterion_group, criterion_main, Criterion, Throughput};
use orderbook::book::OrderBook;
use orderbook::order::{Order, Side};

fn bench_limit_orders(c: &mut Criterion) {
    let mut group = c.benchmark_group("order-book");
    group.throughput(Throughput::Elements(1));

    group.bench_function("limit-order-no-match", |b| {
        let mut book = OrderBook::new();
        let mut id = 0u64;
        b.iter(|| {
            id += 1;
            // Alternating bids and asks that don't cross — pure book insertion
            let side = if id % 2 == 0 { Side::Buy } else { Side::Sell };
            let price = if side == Side::Buy { 9900 + (id % 10) } else { 10100 + (id % 10) };
            black_box(book.process(Order::limit(id, side, price, 100)));
        });
    });

    group.bench_function("limit-order-with-match", |b| {
        b.iter(|| {
            let mut book = OrderBook::new();
            book.process(Order::limit(1, Side::Buy,  10005, 500));
            book.process(Order::limit(2, Side::Sell, 10005, 500));  // crosses immediately
        });
    });

    group.finish();
}

criterion_group!(benches, bench_limit_orders);
criterion_main!(benches);