
use serde::{Deserialize, Serialize};
use rust_decimal::Decimal;

/// The `OrderAction` enum is used to specify the action of an order.
/// The default is an erroneous "Dont" to prevent accidental orders being sent
/// from the default build.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize,Default)]
pub enum OrderAction {
    Buy,
    Sell,
    #[default]
    Dont,
}

#[serde_with::skip_serializing_none]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
/// The `OrderTicket` struct is used to place orders, it is the payload of the order/placeorder endpoint
pub struct OrderTicket {
    ///account username
    pub account_spec: String,
    pub account_id: i64,
    pub cl_ord_id: Option<String>,
    pub action: OrderAction,
    pub symbol: String,
    pub order_qty: i64,
    pub order_type: String,
    pub price: Option<Decimal>,
    pub stop_price: Option<Decimal>,
    pub max_show: Option<Decimal>,
    pub peg_difference: Option<Decimal>,
    pub time_in_force: Option<String>,
    pub expire_time: Option<String>,
    pub text: Option<String>,
    pub activation_time: Option<String>,
    pub custom_tag50: Option<String>,
    /// must be set to true if the order is not being placed by a human
    pub is_automated: bool,
}
impl OrderTicket {
    pub fn market_buy(username:&str,account_id:i64,symbol:&str,qty:i64) -> Self {
        Self {
            account_spec: username.to_string(),
            account_id,
            action: OrderAction::Buy,
            symbol: symbol.to_string(),
            order_qty: qty,
            order_type: "Market".to_string(),
            is_automated: true,
            ..Default::default()
        }
    }
    pub fn market_sell(username:&str,account_id:i64,symbol:&str,qty:i64) -> Self {
        Self {
            account_spec: username.to_string(),
            account_id,
            action : OrderAction::Sell,
            symbol: symbol.to_string(),
            order_qty: qty,
            order_type: "Market".to_string(),
            is_automated: true,
            ..Default::default()
        }
    }
}