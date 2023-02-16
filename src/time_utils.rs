use chrono::{Datelike, TimeZone};
use chrono_tz::Canada::Central;

pub fn calculate_seconds_to_cst_time(hour: u32, minutes: u32) -> u64 {
    let now = chrono::Utc::now();
    let now_central = Central.from_utc_datetime(&now.naive_utc());
    let target_time = Central
        .with_ymd_and_hms(now.year(), now.month(), now.day(), hour, minutes, 0)
        .unwrap();
    let duration = target_time - now_central;
    if target_time > now_central {
        duration.num_seconds() as u64
    } else {
        0
    }
}
