use serde::{Serialize, Deserializer};
use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplayClock {
    #[serde(rename = "t")]
    #[serde(deserialize_with = "deserialize_from_str")]
    pub time: DateTime<Utc>,
    #[serde(rename = "s")]
    pub speed: i64,
}

//"2022-09-15T00:00:00.361Z"
fn deserialize_from_str<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    DateTime::parse_from_rfc3339(&s)
        .map_err(serde::de::Error::custom)
        .map(|dt| dt.with_timezone(&Utc))
}