pub fn utc_now_plus_sec_usize(sec: i64) -> usize {
    (chrono::Utc::now() + chrono::Duration::seconds(sec)).timestamp() as usize
}

pub fn utc_now_plus_min_usize(min: i64) -> usize {
    (chrono::Utc::now() + chrono::Duration::minutes(min)).timestamp() as usize
}

pub fn utc_now_plus_days_usize(days: i64) -> usize {
    (chrono::Utc::now() + chrono::Duration::days(days)).timestamp() as usize
}
