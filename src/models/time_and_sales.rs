use std::sync::Arc;

use rust_decimal::Decimal;
use serde::Deserialize;
use serde::Serialize;
use tokio::sync::RwLock;
pub type TimeAndSalesRWL = Arc<RwLock<Vec<TimeAndSalesItem>>>;



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Copy,Eq, PartialOrd, Ord,Default)]
pub enum OrderAction {
    Buy,
    Sell,
    #[default]
    Unknown,
}
impl OrderAction {
    pub fn is_unknown(&self) -> bool {
        self == &OrderAction::Unknown
    }
}
/// A Time and Sales Item represents a single trade and contains info abot its size, price, time, and buy or sell can be inferred.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TimeAndSalesItem {
    pub historical_id: i64,
    pub action: OrderAction,
    pub qty: i64,
    pub price: Decimal,
    pub bid: Decimal,
    pub ask: Decimal,
    pub timestamp: i64,
    pub receipt_delay: i64,
}
impl TimeAndSalesItem {
    pub fn notional(&self) -> Decimal {
        let unsigned = self.price * Decimal::new(self.qty,0);
        match self.action {
            OrderAction::Buy => unsigned,
            OrderAction::Sell => -unsigned,
            OrderAction::Unknown => Decimal::new(0,0),
        }
    }
}

pub fn new_time_and_sales_rwl(capacity:usize) -> Arc<RwLock<Vec<TimeAndSalesItem>>> {
    Arc::new(RwLock::new(Vec::with_capacity(capacity)))
}