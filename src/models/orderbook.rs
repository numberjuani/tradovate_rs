use std::sync::Arc;

use chrono::DateTime;
use chrono::Utc;
use rayon::prelude::IndexedParallelIterator;
use rayon::prelude::IntoParallelRefMutIterator;
use rayon::prelude::ParallelIterator;
use rayon::slice::ParallelSliceMut;
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

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderBook {
    pub contract_id: i64,
    #[serde(deserialize_with = "parse_timestamp")]
    pub timestamp: DateTime<chrono::Utc>,
    pub bids: Vec<Depth>,
    #[serde(rename = "offers")]
    pub asks: Vec<Depth>,
}
impl OrderBook {
    pub fn normalize(&self) -> Self {
        let mut bids = self.bids.clone();
        let mut asks = self.asks.clone();
        bids.par_sort_unstable_by_key(|x| -x.price);
        bids.par_iter_mut().enumerate().for_each(|(index,bid)| {
            bid.price = Decimal::new(-(index as i64+1),0);
        });
        while bids.len() < 30 {
            bids.push(Depth {
                price: Decimal::new(-(bids.len() as i64+1),0),
                size: 0,
            });
        }
        asks.par_sort_unstable_by_key(|x| x.price);
        asks.par_iter_mut().enumerate().for_each(|(index,ask)| {
            ask.price = Decimal::new(index as i64+1,0);
        });
        while asks.len() < 30 {
            asks.push(Depth {
                price: Decimal::new(asks.len() as i64+1,0),
                size: 0,
            });
        }
        Self {
            contract_id: self.contract_id,
            timestamp: self.timestamp,
            bids,
            asks,
        }
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