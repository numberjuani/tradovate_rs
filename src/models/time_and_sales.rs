use std::sync::Arc;

use rust_decimal::Decimal;
use rust_decimal::prelude::ToPrimitive;
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
    pub base_timestamp: i64,
}
impl TimeAndSalesItem {
    pub fn to_feature(&self) -> Vec<f32> {
        let net_qty = if self.action == OrderAction::Buy {
            self.qty as f32
        } else {
            -self.qty as f32
        };
        vec![
            net_qty,
            self.price.to_f32().unwrap(),
            self.bid.to_f32().unwrap(),
            self.ask.to_f32().unwrap(),
            self.timestamp as f32,
        ]
    }
}
pub fn new_time_and_sales_rwl() -> Arc<RwLock<Vec<TimeAndSalesItem>>> {
    Arc::new(RwLock::new(Vec::new()))
}