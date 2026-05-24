use crate::order::{OrderId, Price, Qty};

#[derive(Debug, Clone, serde::Serialize)]
pub struct Fill {
    pub maker_order_id: OrderId,  
    pub taker_order_id: OrderId,  
    pub price:          Price,
    pub qty:            Qty,
}