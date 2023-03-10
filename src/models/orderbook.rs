
use std::sync::Arc;

use chrono::DateTime;
use chrono::Utc;

use rayon::prelude::IntoParallelRefIterator;
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
    pub fn normalize(&self,total_bid:i64,total_ask:i64) -> Self {
        let mut bids = self.bids.clone();
        let mut asks = self.asks.clone();
        bids.par_sort_unstable_by_key(|x| -x.price);
        asks.par_sort_unstable_by_key(|x| x.price);
        bids.par_iter_mut().for_each(|b| {
            b.size = 100*b.size/total_bid;
        });
        asks.par_iter_mut().for_each(|a| {
            a.size = 100*a.size/total_ask;
        });
        Self {
            contract_id: self.contract_id,
            timestamp: self.timestamp,
            bids,
            asks,
        }
    }
    pub fn normalize_calc(&self) -> Self {
        let mut bids = self.bids.clone();
        let mut asks = self.asks.clone();
        bids.par_sort_unstable_by_key(|x| -x.price);
        asks.par_sort_unstable_by_key(|x| x.price);
        let total_bid = bids.par_iter().map(|x| x.size).sum::<i64>();
        bids.par_iter_mut().for_each(|b| {
            b.size = 100*b.size/total_bid;
        });
        let total_ask = asks.par_iter().map(|x| x.size).sum::<i64>();
        asks.par_iter_mut().for_each(|a| {
            a.size = 100*a.size/total_ask;
        });
        Self {
            contract_id: self.contract_id,
            timestamp: self.timestamp,
            bids,
            asks,
        }
    }
    pub fn to_feature(&self) -> Vec<f32> {
        let mut out = Vec::new();
        let total_bid = self.bids.par_iter().map(|x| x.size).sum::<i64>();
        let total_ask = self.asks.par_iter().map(|x| x.size).sum::<i64>();
        out.push(total_bid as f32);
        out.push(total_ask as f32);
        let ratio = total_bid as f32 / total_ask as f32;
        out.push(ratio);
        let normalized = self.normalize(total_bid,total_ask);
        for i in 0..30 {
            out.push(normalized.bids[i].size as f32);
            out.push(normalized.asks[i].size as f32);
        }
        out
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