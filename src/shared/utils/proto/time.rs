use sea_orm::{prelude::DateTimeWithTimeZone, TryFromU64};

const NANOS_PER_SECOND: i64 = 1000000;

pub fn date_time_utc_to_prost_timestamp(
    date_time_utc: DateTimeWithTimeZone,
) -> prost_types::Timestamp {
    let nanos = date_time_utc.timestamp_nanos();
    let seconds = nanos / NANOS_PER_SECOND;
    let nanos = (nanos % NANOS_PER_SECOND) as i32;
    prost_types::Timestamp { nanos, seconds }
}

/// Becareful when using this function, it will truncate the nanos part of the timestamp.
/// Maybe changes needed not tested yet.
pub fn prost_timestamp_to_date_time_utc(timestamp: prost_types::Timestamp) -> DateTimeWithTimeZone {
    let nanos = timestamp.seconds * NANOS_PER_SECOND + timestamp.nanos as i64;
    DateTimeWithTimeZone::try_from_u64(nanos as u64).unwrap()
}
