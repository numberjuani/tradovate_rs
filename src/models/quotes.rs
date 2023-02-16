use std::sync::Arc;

use chrono::DateTime;
use chrono::Utc;
use rust_decimal::Decimal;
use serde::Deserialize;
use serde::Deserializer;
use serde::Serialize;

pub type QuotesRWL = Arc<tokio::sync::RwLock<Vec<Quotes>>>;

pub fn new_quotes_rwl() -> QuotesRWL {
    Arc::new(tokio::sync::RwLock::new(Vec::new()))
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct Quotes {
    pub quotes: Vec<Quote>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct Quote {
    pub contract_id: i64,
    pub entries: Entries,
    pub id: i64,
    #[serde(deserialize_with = "parse_timestamp")]
    pub timestamp: DateTime<chrono::Utc>,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct Entries {
    #[serde(rename = "Bid")]
    pub bid: PriceSize,
    #[serde(rename = "HighPrice")]
    pub high_price: HighPrice,
    #[serde(rename = "LowPrice")]
    pub low_price: LowPrice,
    #[serde(rename = "Offer")]
    pub offer: PriceSize,
    #[serde(rename = "OpenInterest")]
    pub open_interest: OpenInterest,
    #[serde(rename = "OpeningPrice")]
    pub opening_price: OpeningPrice,
    #[serde(rename = "SettlementPrice")]
    pub settlement_price: SettlementPrice,
    #[serde(rename = "TotalTradeVolume")]
    pub total_trade_volume: TotalTradeVolume,
    #[serde(rename = "Trade")]
    pub trade: PriceSize
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct PriceSize {
    pub price: Decimal,
    pub size: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HighPrice {
    pub price: Decimal,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LowPrice {
    pub price: Decimal,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenInterest {
    pub size: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpeningPrice {
    pub price: Decimal,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SettlementPrice {
    pub price: Decimal,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TotalTradeVolume {
    pub size: i64,
}


//"2022-09-15T00:00:58.230Z"
fn parse_timestamp<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    let dt = DateTime::parse_from_rfc3339(&s).map_err(serde::de::Error::custom)?;
    Ok(dt.with_timezone(&Utc))
}