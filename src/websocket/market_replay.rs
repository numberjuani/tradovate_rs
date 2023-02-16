use chrono::Utc;
use serde::Serialize;
use serde::Deserialize;
use chrono::DateTime;


#[derive(Serialize, Deserialize, Debug,Default)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct MarketReplaySettings {
    #[serde(serialize_with = "parse_mr_date")]
    pub start_timestamp: DateTime<Utc>,
    pub speed: i64,
    pub initial_balance: i64,
}
impl MarketReplaySettings {
    pub fn to_request(&self,request_num:i64) -> String {
        format!("replay/initializeclock\n{}\n\n{}",request_num,serde_json::to_string(self).unwrap())
    }
}


//"%Y-%m-%dT%H:%M:%S%.fZ"
pub fn parse_mr_date<S>(date: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let s = format!("{}", date.format("%Y-%m-%dT%H:%M:%S%.3f%.fZ"));
    serializer.serialize_str(&s)
}