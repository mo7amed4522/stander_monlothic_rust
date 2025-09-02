//! Date and time utilities

use chrono::{DateTime, Utc, Duration, TimeZone};
use anyhow::Result;


pub fn now_utc() -> DateTime<Utc> {
    Utc::now()
}
pub fn format_iso8601(dt: &DateTime<Utc>) -> String {
    dt.to_rfc3339()
}
pub fn parse_iso8601(s: &str) -> Result<DateTime<Utc>> {
    DateTime::parse_from_rfc3339(s)
        .map(|dt| dt.with_timezone(&Utc))
        .map_err(|e| anyhow::anyhow!("Failed to parse datetime: {}", e))
}
pub fn format_display(dt: &DateTime<Utc>) -> String {
    dt.format("%Y-%m-%d %H:%M:%S UTC").to_string()
}
pub fn format_filename(dt: &DateTime<Utc>) -> String {
    dt.format("%Y%m%d_%H%M%S").to_string()
}
pub fn start_of_day(dt: &DateTime<Utc>) -> DateTime<Utc> {
    dt.date_naive().and_hms_opt(0, 0, 0)
        .map(|naive| Utc.from_utc_datetime(&naive))
        .unwrap_or(*dt)
}
pub fn end_of_day(dt: &DateTime<Utc>) -> DateTime<Utc> {
    dt.date_naive().and_hms_opt(23, 59, 59)
        .map(|naive| Utc.from_utc_datetime(&naive))
        .unwrap_or(*dt)
}
pub fn add_duration(dt: &DateTime<Utc>, duration: Duration) -> DateTime<Utc> {
    *dt + duration
}
pub fn subtract_duration(dt: &DateTime<Utc>, duration: Duration) -> DateTime<Utc> {
    *dt - duration
}
pub fn duration_between(start: &DateTime<Utc>, end: &DateTime<Utc>) -> Duration {
    *end - *start
}
pub fn is_past(dt: &DateTime<Utc>) -> bool {
    *dt < now_utc()
}
pub fn is_future(dt: &DateTime<Utc>) -> bool {
    *dt > now_utc()
}
pub fn age_in_days(dt: &DateTime<Utc>) -> i64 {
    let now = now_utc();
    (now - *dt).num_days()
}
pub fn age_in_hours(dt: &DateTime<Utc>) -> i64 {
    let now = now_utc();
    (now - *dt).num_hours()
}
pub fn age_in_minutes(dt: &DateTime<Utc>) -> i64 {
    let now = now_utc();
    (now - *dt).num_minutes()
}
pub fn from_timestamp(timestamp: i64) -> Result<DateTime<Utc>> {
    Utc.timestamp_opt(timestamp, 0)
        .single()
        .ok_or_else(|| anyhow::anyhow!("Invalid timestamp: {}", timestamp))
}
pub fn to_timestamp(dt: &DateTime<Utc>) -> i64 {
    dt.timestamp()
}
pub mod durations {
    use chrono::Duration;
    pub fn seconds(n: i64) -> Duration {
        Duration::seconds(n)
    }
    pub fn minutes(n: i64) -> Duration {
        Duration::minutes(n)
    }
    pub fn hours(n: i64) -> Duration {
        Duration::hours(n)
    }
    pub fn days(n: i64) -> Duration {
        Duration::days(n)
    }
    pub fn weeks(n: i64) -> Duration {
        Duration::weeks(n)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{TimeZone, Timelike};
    #[test]
    fn test_format_and_parse() {
        let dt = Utc.with_ymd_and_hms(2023, 12, 25, 12, 30, 45).unwrap();
        let iso_string = format_iso8601(&dt);
        let parsed = parse_iso8601(&iso_string).unwrap();
        assert_eq!(dt, parsed);
    }
    #[test]
    fn test_start_and_end_of_day() {
        let dt = Utc.with_ymd_and_hms(2023, 12, 25, 15, 30, 45).unwrap();
        let start = start_of_day(&dt);
        let end = end_of_day(&dt);

        assert_eq!(start.hour(), 0);
        assert_eq!(start.minute(), 0);
        assert_eq!(start.second(), 0);

        assert_eq!(end.hour(), 23);
        assert_eq!(end.minute(), 59);
        assert_eq!(end.second(), 59);
    }
    #[test]
    fn test_duration_operations() {
        let dt = now_utc();
        let future = add_duration(&dt, durations::hours(2));
        let past = subtract_duration(&dt, durations::hours(1));
        assert!(is_future(&future));
        assert!(is_past(&past));
        let duration = duration_between(&past, &future);
        assert_eq!(duration.num_hours(), 3);
    }

    #[test]
    fn test_timestamp_conversion() {
        let timestamp = 1703505045;
        let dt = from_timestamp(timestamp).unwrap();
        let back_to_timestamp = to_timestamp(&dt);
        assert_eq!(timestamp, back_to_timestamp);
    }
}
