use serde::{Deserialize, Serialize};

pub type Balances = Vec<Balance>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Balance {
    pub account_id: i64,
    pub amount: f64,
    #[serde(rename = "amountSOD")]
    pub amount_sod: f64,
    pub archived: bool,
    pub currency_id: i64,
    pub id: i64,
    pub realized_pn_l: f64,
    pub timestamp: String,
    pub trade_date: TradeDate,
    pub week_realized_pn_l: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TradeDate {
    pub day: i64,
    pub month: i64,
    pub year: i64,
}
