use anyhow::{anyhow, Result};
use chrono::{DateTime, NaiveDateTime, Utc};
use sea_orm::prelude::DateTimeWithTimeZone;

pub struct LettDate {}

impl LettDate {
    pub fn utc_now() -> chrono::DateTime<chrono::Utc> {
        chrono::Utc::now()
    }

    pub fn dt_with_tz(ts: usize) -> Result<DateTimeWithTimeZone> {
        let native_date_time = NaiveDateTime::from_timestamp_millis(ts as i64)
            .ok_or(anyhow!("Invalid Timestamp millis"))?;
        let date_time_utc: DateTime<Utc> = DateTime::from_utc(native_date_time, Utc);

        Ok(DateTimeWithTimeZone::from(date_time_utc))
    }

    pub fn now_dt_with_tz() -> DateTimeWithTimeZone {
        let date_time_utc: DateTime<Utc> = Self::utc_now();

        DateTimeWithTimeZone::from(date_time_utc)
    }
}
