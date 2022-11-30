use chrono::DateTime;
use chrono::FixedOffset;
use serde::Deserialize;
use serde::Serialize;
use crate::utils;



#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
#[serde(rename_all = "camelCase")]
pub struct AccessTokenInfo {
    pub access_token: String,
    pub md_access_token: String,
    #[serde(deserialize_with = "utils::fixed_offset_date_time_from_str")]
    pub expiration_time: DateTime<FixedOffset>,
    pub user_status: String,
    pub user_id: i64,
    pub name: String,
    pub has_live: bool,
    pub outdated_ta_c: bool,
    pub has_funded: bool,
    pub has_market_data: bool,
    pub outdated_liquidation_policy: bool,
}
impl AccessTokenInfo {
    pub fn is_expired(&self) -> bool {
        self.expiration_time < chrono::Utc::now()
    }
}




