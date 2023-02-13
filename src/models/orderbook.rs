use chrono::DateTime;
use chrono::Utc;
use serde::Deserialize;
use serde::Deserializer;
use serde::Serialize;
use serde::de;
use rust_decimal::Decimal;

 
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
    pub offers: Vec<Depth>,
 }
 
 #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
 #[serde(rename_all = "camelCase")]
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