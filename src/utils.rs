use chrono::{DateTime, FixedOffset};
use serde::{Deserializer, Deserialize, de};
use serde_json::Value;

pub fn open_json(filename: &str) -> Result<Value, std::io::Error> {
    let file = std::fs::File::open(filename)?;
    let reader = std::io::BufReader::new(file);
    let data_file: Value = serde_json::from_reader(reader)?;
    Ok(data_file)
}

pub fn create_json_file<T: serde::Serialize>(filename: &str, contents: &T) {
    serde_json::to_writer(&std::fs::File::create(filename).unwrap(), contents).unwrap();
}

pub fn delete_file(filename: &str) {
    std::fs::remove_file(filename).unwrap()
}

pub fn fixed_offset_date_time_from_str<'de, D>(deserializer: D) -> Result<DateTime<FixedOffset>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    chrono::DateTime::parse_from_rfc3339(&s).map_err(de::Error::custom)
}