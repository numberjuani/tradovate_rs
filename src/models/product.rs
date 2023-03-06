use serde::Deserialize;
use serde::Serialize;
use rust_decimal::Decimal;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Product {
    pub id: i64,
    pub name: String,
    pub currency_id: i64,
    pub product_type: String,
    pub description: String,
    pub exchange_id: i64,
    pub exchange_channel_id: i64,
    pub contract_group_id: i64,
    pub risk_discount_contract_group_id: Option<i64>,
    pub status: String,
    pub months: Option<String>,
    pub value_per_point: Decimal,
    pub price_format_type: String,
    pub price_format: i64,
    pub tick_size: Decimal,
    pub allow_provider_contract_info: bool,
    pub is_micro: bool,
    pub market_data_source: String,
    pub underlying_id: Option<i64>,
    pub lookup_weight: Option<i64>,
    pub has_replay: Option<bool>,
    pub continuous_rollover_days: Option<i64>,
    pub spread_type: Option<String>,
    pub rollover_months: Option<String>,
    pub strike_format: Option<i64>,
    pub strike_display_multiplier: Option<Decimal>,
    pub settlement_method: Option<String>,
    pub event_payout: Option<Decimal>,
    pub underlying_reference_id: Option<i64>,
    pub is_secured: Option<bool>,
}
impl Product {
    pub fn round_price_to_nearest_tick(&self, price: Decimal) -> Decimal {
        let price = price / self.tick_size;
        let price = price.round_dp(0);
        let price = price * self.tick_size;
        price
    }
}