use chrono::{DateTime, FixedOffset, NaiveDateTime};

pub fn format_timestamp_to_datetime(timestamp: i64) -> String {
    let nt = NaiveDateTime::from_timestamp_opt(timestamp, 0).unwrap();
    let t = DateTime::<FixedOffset>::from_utc(nt, FixedOffset::east_opt(8 * 3600).unwrap());
    let st = t.format("%Y-%m-%d %H:%M:%S");
    st.to_string()
}

pub fn format_timestamp_to_date(timestamp: i64) -> String {
    let nt = NaiveDateTime::from_timestamp_opt(timestamp, 0).unwrap();
    let t = DateTime::<FixedOffset>::from_utc(nt, FixedOffset::east_opt(8 * 3600).unwrap());
    let st = t.format("%Y-%m-%d");
    st.to_string()
}
