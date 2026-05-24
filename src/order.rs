use std::time::{SystemTime, UNIX_EPOCH};

pub type OrderId = u64;
pub type Price   = u64;  // Store as integer cents to avoid float comparison bugs
pub type Qty     = u64;

#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize)]
pub enum Side { 
    Buy, 
    Sell 
}

#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize)]
pub enum OrderType { 
    Limit, 
    Market 
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct Order {
    pub id:         OrderId,
    pub side:       Side,
    pub order_type: OrderType,
    pub price:      Price,
    pub qty:        Qty,
    pub timestamp:  u64,
}

impl Order {
    pub fn limit(id: OrderId, side: Side, price: Price, qty: Qty) -> Self {
        Self { id, side, order_type: OrderType::Limit, price, qty, timestamp: now_ns() }
    }
    pub fn market(id: OrderId, side: Side, qty: Qty) -> Self {
        Self { id, side, order_type: OrderType::Market, price: 0, qty, timestamp: now_ns() }
    }
}

fn now_ns() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos() as u64
}