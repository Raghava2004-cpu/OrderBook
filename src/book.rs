use std::collections::{BTreeMap, VecDeque, HashMap};
use crate::order::{Order, OrderId, Price, Side, OrderType};
use crate::fills::Fill;

// A price level holds all orders at that price, in FIFO order
type Level = VecDeque<Order>;

pub struct OrderBook {
    bids: BTreeMap<Price, Level>,
    asks: BTreeMap<Price, Level>,
    index: HashMap<OrderId, (Side, Price)>,
}

impl OrderBook {
    pub fn new() -> Self {
        Self {
            bids:  BTreeMap::new(),
            asks:  BTreeMap::new(),
            index: HashMap::new(),
        }
    }

    pub fn process(&mut self, order: Order) -> Vec<Fill> {
        match order.order_type {
            OrderType::Market => self.match_market(order),
            OrderType::Limit  => self.match_limit(order),
        }
    }

    pub fn cancel(&mut self, id: OrderId) -> bool {
        if let Some((side, price)) = self.index.remove(&id) {
            let book = match side { Side::Buy => &mut self.bids, Side::Sell => &mut self.asks };
            if let Some(level) = book.get_mut(&price) {
                level.retain(|o| o.id != id);
                if level.is_empty() {
                    book.remove(&price);
                }
                return true;
            }
        }
        false
    }

    pub fn best_bid(&self) -> Option<Price> {
        self.bids.keys().next_back().copied()
    }

    pub fn best_ask(&self) -> Option<Price> {
        self.asks.keys().next().copied()
    }

    pub fn spread(&self) -> Option<i64> {
        match (self.best_bid(), self.best_ask()) {
            (Some(bid), Some(ask)) => Some((ask as i64) - (bid as i64)),
            _ => None,
        }
    }

    fn match_market(&mut self, mut incoming: Order) -> Vec<Fill> {
        let mut fills = Vec::new();
        match incoming.side {
            Side::Buy  => self.match_against_asks(&mut incoming, &mut fills),
            Side::Sell => self.match_against_bids(&mut incoming, &mut fills),
        }
        fills
    }

    fn match_limit(&mut self, mut incoming: Order) -> Vec<Fill> {
        let mut fills = Vec::new();
        match incoming.side {
            Side::Buy  => self.match_against_asks(&mut incoming, &mut fills),
            Side::Sell => self.match_against_bids(&mut incoming, &mut fills),
        }
        if incoming.qty > 0 {
            self.rest(incoming);
        }
        fills
    }

    fn match_against_asks(&mut self, incoming: &mut Order, fills: &mut Vec<Fill>) {
        while incoming.qty > 0 {
            let &best_ask = match self.asks.keys().next() {
                Some(p) => p,
                None => break,
            };
            if incoming.order_type == OrderType::Limit && incoming.price < best_ask {
                break;
            }
            if let Some(level) = self.asks.get_mut(&best_ask) {
                Self::drain_level(level, incoming, best_ask, fills, &mut self.index);
                if level.is_empty() {
                    self.asks.remove(&best_ask);
                }
            }
        }
    }

    fn match_against_bids(&mut self, incoming: &mut Order, fills: &mut Vec<Fill>) {
        while incoming.qty > 0 {
            let &best_bid = match self.bids.keys().next_back() {
                Some(p) => p,
                None => break,
            };
            if incoming.order_type == OrderType::Limit && incoming.price > best_bid {
                break;
            }
            if let Some(level) = self.bids.get_mut(&best_bid) {
                Self::drain_level(level, incoming, best_bid, fills, &mut self.index);
                if level.is_empty() {
                    self.bids.remove(&best_bid);
                }
            }
        }
    }

    fn drain_level(
        level:   &mut Level,
        taker:   &mut Order,
        price:   Price,
        fills:   &mut Vec<Fill>,
        index:   &mut HashMap<OrderId, (Side, Price)>,
    ) {
        while let Some(maker) = level.front_mut() {
            if taker.qty == 0 { break; }

            let trade_qty = taker.qty.min(maker.qty);
            fills.push(Fill {
                maker_order_id: maker.id,
                taker_order_id: taker.id,
                price,
                qty: trade_qty,
            });

            taker.qty -= trade_qty;
            maker.qty -= trade_qty;

            if maker.qty == 0 {
                index.remove(&maker.id);
                level.pop_front();
            }
        }
    }

    fn rest(&mut self, order: Order) {
        let (book, side) = match order.side {
            Side::Buy  => (&mut self.bids, Side::Buy),
            Side::Sell => (&mut self.asks, Side::Sell),
        };
        self.index.insert(order.id, (side, order.price));
        book.entry(order.price).or_default().push_back(order);
    }
}