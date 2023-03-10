use std::sync::Arc;

use rust_decimal::Decimal;
use serde::Deserialize;
use serde::Serialize;
use tokio::sync::RwLock;

use super::tick_chart::ChartSummary;
pub type TimeAndSalesRWL = Arc<RwLock<Vec<ChartSummary>>>;




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
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize,Default)]
#[serde(default)]
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
    pub fn net_qty(&self) -> i64 {
        if self.action == OrderAction::Buy {
            self.qty
        } else {
            -self.qty
        }
    }
}
pub fn new_time_and_sales_rwl() -> Arc<RwLock<Vec<ChartSummary>>> {
    Arc::new(RwLock::new(Vec::new()))
}