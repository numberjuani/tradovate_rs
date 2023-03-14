

use serde::{Deserialize, Serialize};
use serde_json::Value;


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserSyncMessage {
    #[serde(rename = "s")]
    pub status: i64,
    #[serde(rename = "i")]
    pub request_number: i64,
    #[serde(rename = "d")]
    pub user_data: UserData,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserData {
    pub users: Vec<User>,
    pub accounts: Vec<Account>,
    pub account_risk_statuses: Vec<AccountRiskStatuse>,
    pub margin_snapshots: Vec<MarginSnapshot>,
    pub user_account_auto_liqs: Vec<UserAccountAutoLiq>,
    pub cash_balances: Vec<CashBalance>,
    pub currencies: Vec<Currency>,
    pub positions: Vec<Value>,
    pub fill_pairs: Vec<Value>,
    pub orders: Vec<Value>,
    pub contracts: Vec<Value>,
    pub contract_maturities: Vec<Value>,
    pub products: Vec<Value>,
    pub exchanges: Vec<Exchange>,
    pub spread_definitions: Vec<Value>,
    pub commands: Vec<Value>,
    pub command_reports: Vec<Value>,
    pub execution_reports: Vec<Value>,
    pub order_versions: Vec<Value>,
    pub fills: Vec<Value>,
    pub order_strategies: Vec<Value>,
    pub order_strategy_links: Vec<Value>,
    pub user_properties: Vec<UserProperty>,
    pub properties: Vec<Value>,
    pub user_plugins: Vec<UserPlugin>,
    pub user_read_statuses: Vec<UserReadStatuse>,
    pub contract_groups: Vec<ContractGroup>,
    pub order_strategy_types: Vec<OrderStrategyType>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: i64,
    pub name: String,
    pub timestamp: String,
    pub user_type: String,
    pub email: String,
    pub status: String,
    pub creation_timestamp: String,
    pub professional: bool,
    pub two_factor_auth: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Account {
    pub id: i64,
    pub name: String,
    pub user_id: i64,
    pub account_type: String,
    pub active: bool,
    pub clearing_house_id: i64,
    pub risk_category_id: i64,
    pub auto_liq_profile_id: i64,
    pub margin_account_type: String,
    pub legal_status: String,
    pub archived: bool,
    pub timestamp: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountRiskStatuse {
    pub id: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarginSnapshot {
    pub id: i64,
    pub timestamp: String,
    pub risk_time_period_id: i64,
    pub initial_margin: f64,
    pub maintenance_margin: f64,
    pub auto_liq_level: f64,
    pub liq_only_level: f64,
    pub total_used_margin: f64,
    pub full_initial_margin: f64,
    pub position_margin: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserAccountAutoLiq {
    pub id: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CashBalance {
    pub id: i64,
    pub account_id: i64,
    pub timestamp: String,
    pub trade_date: TradeDate,
    pub currency_id: i64,
    pub amount: f64,
    pub realized_pn_l: f64,
    pub week_realized_pn_l: f64,
    pub archived: bool,
    #[serde(rename = "amountSOD")]
    pub amount_sod: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TradeDate {
    pub year: i64,
    pub month: i64,
    pub day: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Currency {
    pub id: i64,
    pub name: String,
    pub symbol: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Exchange {
    pub id: i64,
    pub name: String,
    pub complex: String,
    pub time_zone: String,
    pub is_secured_default: bool,
    pub cftc_reporting: bool,
    pub free_market_data: String,
    pub market_type: String,
    pub span: Option<String>,
    pub foreign_exchange: bool,
    pub tradingview_groups: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserProperty {
    pub id: i64,
    pub user_id: i64,
    pub property_id: i64,
    pub value: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserPlugin {
    pub id: i64,
    pub user_id: i64,
    pub timestamp: String,
    pub plan_price: f64,
    pub archived: bool,
    pub plugin_name: String,
    pub approval: bool,
    pub entitlement_id: Option<i64>,
    pub start_date: StartDate,
    pub paid_amount: f64,
    pub plan_categories: Option<String>,
    pub credit_card_transaction_id: Option<i64>,
    pub credit_card_id: Option<i64>,
    pub expiration_date: Option<ExpirationDate>,
    pub autorenewal: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StartDate {
    pub year: i64,
    pub month: i64,
    pub day: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExpirationDate {
    pub year: i64,
    pub month: i64,
    pub day: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserReadStatuse {
    pub id: i64,
    pub news_story_id: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContractGroup {
    pub id: i64,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderStrategyType {
    pub id: i64,
    pub name: String,
    pub enabled: bool,
}
