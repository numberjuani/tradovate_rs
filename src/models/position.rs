use serde::Deserialize;
use serde::Serialize;
use rust_decimal::Decimal;
use chrono::DateTime;
use crate::models::position::parse_timestamp::parse_timestamp;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Position {
    pub id: i64,
    pub account_id: i64,
    pub contract_id: i64,
    #[serde(deserialize_with = "parse_timestamp")]
    pub timestamp: DateTime<chrono::Utc>,
    pub trade_date: TradeDate,
    pub net_pos: i64,
    pub net_price: f64,
    pub bought: i64,
    pub bought_value: Decimal,
    pub sold: i64,
    pub sold_value: Decimal,
    pub archived: bool,
    pub prev_pos: i64,
}
impl Position {
    pub fn seconds_elapsed_since_opened(&self) -> i64 {
        chrono::Utc::now().signed_duration_since(self.timestamp).num_seconds()
    }
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TradeDate {
    pub year: i64,
    pub month: i64,
    pub day: i64,
}

//2023-02-13T18:47:25.552Z
mod parse_timestamp {
    use chrono::DateTime;
    use chrono::Utc;
    use serde::Deserialize;
    use serde::de;
    use serde::Deserializer;

    pub fn parse_timestamp<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = Deserialize::deserialize(deserializer)?;
        let dt = chrono::NaiveDateTime::parse_from_str(&s, "%Y-%m-%dT%H:%M:%S%.fZ")
            .map_err(de::Error::custom)?;
        Ok(chrono::DateTime::<Utc>::from_utc(dt, Utc))
    }
}