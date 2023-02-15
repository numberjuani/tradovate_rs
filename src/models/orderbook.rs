use std::sync::Arc;

use chrono::DateTime;
use chrono::Utc;
use rust_decimal::Decimal;
use serde::de;
use serde::Deserialize;
use serde::Deserializer;
use serde::Serialize;
use tokio::sync::RwLock;
pub type OrderBooksRWL = Arc<RwLock<Vec<OrderBooks>>>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderBooks {
    pub doms: Vec<OrderBook>,
}
impl OrderBooks {
    pub fn to_csv_format(&self) -> String {
        let mut csv = String::new();
        for book in &self.doms {
            csv.push_str(&book.to_csv_format());
        }
        csv
    }
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderBook {
    pub contract_id: i64,
    #[serde(deserialize_with = "parse_timestamp")]
    pub timestamp: DateTime<chrono::Utc>,
    pub bids: Vec<Depth>,
    pub offers: Vec<Depth>,
}
impl OrderBook {
    pub fn to_csv_format(&self) -> String {
        //sort the asks in ascending order
        let mut csv = String::new();
        let mut asks = self.offers.clone();
        asks.sort_unstable_by_key(|a| a.price);
        //sort the bids in descending order
        let mut bids = self.bids.clone();
        bids.sort_unstable_by_key(|b| b.price);
        let asks = asks
            .iter()
            .map(|a| format!("{}" ,-a.size))
            .collect::<Vec<String>>()
            .join(",");
        let bids = bids
            .iter()
            .map(|b| format!("{}" ,b.size))
            .collect::<Vec<String>>()
            .join(",");
        let row = format!("{},{},{},{}\n", asks, bids, self.timestamp.timestamp_millis(),self.contract_id);
        csv.push_str(&row);
        csv
    }
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct Depth {
    pub price: Decimal,
    pub size: i64,
}

pub fn parse_timestamp<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    let dt = chrono::NaiveDateTime::parse_from_str(&s, "%Y-%m-%dT%H:%M:%S%.fZ")
        .map_err(de::Error::custom)?;
    Ok(chrono::DateTime::<Utc>::from_utc(dt, Utc))
}


pub fn new_orderbooks_rwl() -> Arc<RwLock<Vec<OrderBooks>>> {
    Arc::new(RwLock::new(Vec::new()))
}