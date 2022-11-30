use serde::Deserialize;
use serde::Serialize;



#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Position {
    pub id: i64,
    pub account_id: i64,
    pub contract_id: i64,
    pub timestamp: String,
    pub trade_date: TradeDate,
    pub net_pos: i64,
    pub net_price: f64,
    pub bought: i64,
    pub bought_value: f64,
    pub sold: i64,
    pub sold_value: f64,
    pub archived: bool,
    pub prev_pos: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TradeDate {
    pub year: i64,
    pub month: i64,
    pub day: i64,
}
