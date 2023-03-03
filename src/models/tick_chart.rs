use rayon::prelude::IndexedParallelIterator;
use rayon::prelude::IntoParallelRefIterator;
use rayon::prelude::ParallelIterator;
use serde::Serialize;
use serde::Deserialize;
use chrono::NaiveDate;
use rust_decimal::Decimal;
use super::time_and_sales::OrderAction;
use super::time_and_sales::TimeAndSalesItem;
use std::cmp::Ordering;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct ChartData {
    pub charts: Vec<Chart>,
}
impl ChartData {
    pub fn get_all_ts_items(&self) -> Vec<TimeAndSalesItem> {
        let mut vec_of_vec = Vec::new();
        self.charts
            .par_iter()
            .map(|chart| chart.get_ts_items())
            .collect_into_vec(&mut vec_of_vec);
        vec_of_vec.into_iter().flatten().collect()
    }
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
#[serde(default)]
pub struct Chart {
    pub contract_id: i64,
    #[serde(rename = "bp")]
    pub base_price: i64,
    #[serde(rename = "bt")]
    pub base_timestamp: i64,
    #[serde(rename = "id")]
    pub historical_id: i64,
    #[serde(rename = "s")]
    pub packet_data_source: String,
    #[serde(rename = "td")]
    #[serde(deserialize_with = "parse_trade_date")]
    pub trade_date: NaiveDate,
    #[serde(rename = "tks")]
    pub ticks: Vec<Tick>,
    #[serde(rename = "ts")]
    pub tick_size: Decimal,
    pub eoh: bool,
}
impl Chart {
    pub fn get_ts_items(&self) -> Vec<TimeAndSalesItem> {
        let mut output_vec = Vec::new();
        let base_price = Decimal::new(self.base_price,0) * self.tick_size;
        self.ticks
            .par_iter()
            .map(|tick| {
                tick.to_ts_item(
                    base_price,
                    self.tick_size,
                    self.base_timestamp,
                    self.historical_id,
                )
            })
            .collect_into_vec(&mut output_vec);
        output_vec
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
#[serde(default)]
pub struct Tick {
    #[serde(rename = "a")]
    pub ask_relative_price: i64,
    #[serde(rename = "as")]
    pub ask_size: i64,
    #[serde(rename = "b")]
    pub bid_relative_price: i64,
    #[serde(rename = "bs")]
    pub bid_size: i64,
    #[serde(rename = "id")]
    pub tick_id: i64,
    #[serde(rename = "p")]
    pub relative_price: i64, // Actual tick price is packet.bp + tick.p
    #[serde(rename = "s")]
    pub tick_volume: i64,
    #[serde(rename = "t")]
    pub relative_timestamp: i64, // Actual tick timestamp is packet.bt + tick.t
}
impl Tick {
    pub fn to_ts_item(
        &self,
        base_price: Decimal,
        tick_size: Decimal,
        base_timestamp: i64,
        historical_id: i64,
    ) -> TimeAndSalesItem {
        let price = base_price + (tick_size * Decimal::new(self.relative_price,0));
        let bid = base_price + (tick_size * Decimal::new(self.bid_relative_price,0));
        let ask = base_price + (tick_size * Decimal::new(self.ask_relative_price,0));
        let mid_price = (ask + bid)/Decimal::new(2,0);
        let timestamp = base_timestamp + self.relative_timestamp;
        let delay = chrono::Utc::now().timestamp_millis() - timestamp;
        let action = match price.cmp(&mid_price) {
            Ordering::Greater => OrderAction::Buy,
            Ordering::Less => OrderAction::Sell,
            Ordering::Equal => OrderAction::Unknown,
        };
        TimeAndSalesItem {
            historical_id,
            action,
            qty: self.tick_volume,
            price,
            timestamp,
            receipt_delay: delay,
            bid,
            ask,
            base_timestamp,
        }
    }
}

fn parse_trade_date<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s: i64 = Deserialize::deserialize(deserializer)?;
    let s = s.to_string();
    let dt = NaiveDate::parse_from_str(&s, "%Y%m%d")
        .map_err(serde::de::Error::custom)?;
    Ok(dt)
}