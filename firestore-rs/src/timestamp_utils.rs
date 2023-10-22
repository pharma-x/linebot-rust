use crate::errors::*;
use crate::FirestoreResult;
use chrono::prelude::*;

pub fn from_timestamp(ts: prost_types::Timestamp) -> FirestoreResult<DateTime<Local>> {
    if let Some(dt) = chrono::NaiveDateTime::from_timestamp_opt(ts.seconds, ts.nanos as u32) {
        Ok(DateTime::<Local>::from_naive_utc_and_offset(
            dt,
            Local.offset_from_utc_datetime(&dt),
        ))
    } else {
        Err(FirestoreError::DeserializeError(
            FirestoreSerializationError::from_message(format!(
                "Invalid or out-of-range datetime: {ts}"
            )),
        ))
    }
}

pub fn to_timestamp(dt: DateTime<Local>) -> prost_types::Timestamp {
    prost_types::Timestamp {
        seconds: dt.timestamp(),
        nanos: dt.nanosecond() as i32,
    }
}
