use rayon::prelude::IndexedParallelIterator;
use rayon::prelude::IntoParallelRefIterator;
use rayon::prelude::ParallelIterator;
use rayon::slice::ParallelSliceMut;
use rust_decimal::prelude::ToPrimitive;
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
    pub fn combine_all_ticks(&self) -> Option<ChartSummary>  {
        let mut items = self.get_all_ts_items();
        items.par_sort_unstable_by_key(|i| i.timestamp);
        let mut net_qty = 0;
        let mut abs_qty = 0;
        let mut biggest_buy = 0;
        let mut biggest_sell = 0;
        if items.len() >= 2 {
            for item in &items {
                net_qty += item.net_qty();
                abs_qty += item.qty.abs();
                match item.action {
                    OrderAction::Buy => {
                        if item.qty > biggest_buy {
                            biggest_buy = item.qty;
                        }
                    }
                    OrderAction::Sell => {
                        if item.qty  > biggest_sell {
                            biggest_sell = item.qty;
                        }
                    }
                    OrderAction::Unknown => {
                        continue;
                    },
                }
            }
            let mean_net_qty = net_qty / items.len() as i64;
            let mean_abs_qty = abs_qty / items.len() as i64;
            let last_item = items.last().unwrap();
            let timespan = last_item.timestamp - items.first().unwrap().timestamp;
            let last_timestamp = last_item.timestamp;
            let last_bid = last_item.bid;
            let last_ask = last_item.ask;
            Some(ChartSummary {
                net_qty,
                mean_net_qty,
                abs_qty,
                mean_abs_qty,
                biggest_buy,
                biggest_sell,
                timespan,
                last_timestamp,
                last_bid,
                last_ask,
                num_ticks: items.len(),
                last_price: last_item.price,
            })
        } else if items.len() ==1   {
            Some(ChartSummary {
                net_qty : items[0].net_qty(),
                mean_net_qty: items[0].net_qty(),
                abs_qty: items[0].qty.abs(),
                mean_abs_qty: items[0].qty.abs(),
                biggest_buy : if items[0].action == OrderAction::Buy {items[0].qty.abs()} else {0},
                biggest_sell : if items[0].action == OrderAction::Sell {items[0].qty.abs()} else {0},
                timespan: 0,
                num_ticks: 1,
                last_bid: items[0].bid,
                last_ask: items[0].ask,
                last_timestamp: items[0].timestamp,
                last_price: items[0].price,
            })
        } else {
            None
        }
        
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

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChartSummary {
    pub net_qty: i64,
    pub mean_net_qty: i64,
    pub abs_qty: i64,
    pub mean_abs_qty: i64,
    pub biggest_buy: i64,
    pub biggest_sell: i64,
    pub timespan: i64,
    pub num_ticks: usize,
    pub last_bid: Decimal,
    pub last_ask: Decimal,
    pub last_timestamp: i64,
    pub last_price: Decimal,
}
impl ChartSummary {
    pub fn to_features(&self) -> Vec<f32> {
        vec![
            self.net_qty as f32,
            self.mean_net_qty as f32,
            self.abs_qty as f32,
            self.mean_abs_qty as f32,
            self.biggest_buy as f32,
            self.biggest_sell as f32,
            self.timespan as f32,
            self.num_ticks as f32,
            self.last_bid.to_f32().unwrap(),
            self.last_ask.to_f32().unwrap(),
            self.last_timestamp as f32,
            self.last_price.to_f32().unwrap(),
        ]
    }
}

/*
0: net qty
1: mean net qty
2: abs qty
3: mean abs qty
4: biggest buy
5: biggest sell
6: timespan
7: num ticks
8: last bid
9: last ask
10: last timestamp
11: last price
...
 */