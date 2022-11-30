use chrono::DateTime;
use chrono::FixedOffset;
use serde::Deserialize;
use serde::Deserializer;
use serde::Serialize;
use serde::de;



#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Contract {
    pub id: i64,
    pub name: String,
    pub contract_maturity_id: i64,
    pub status: String,
    pub provider_tick_size: f64,
}


#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Maturity {
    pub id: i64,
    pub product_id: i64,
    pub expiration_month: i64,
    #[serde(deserialize_with = "parse_exp_date")]
    pub expiration_date: DateTime<FixedOffset>,
    pub archived: bool,
    pub seq_no: i64,
    pub is_front: bool,
}
impl Maturity {
    pub fn days_to_expiration(&self) -> i64 {
        self.expiration_date.signed_duration_since(chrono::Utc::now()).num_days()
    }
}
//"2022-12-16T21:15Z"
pub fn parse_exp_date<'de, D>(deserializer: D) -> Result<DateTime<FixedOffset>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    let dt = chrono::NaiveDateTime::parse_from_str(&s, "%Y-%m-%dT%H:%M%Z").map_err(de::Error::custom)?;
    Ok(chrono::DateTime::<FixedOffset>::from_utc(dt, FixedOffset::east_opt(0).unwrap()))

}
    
