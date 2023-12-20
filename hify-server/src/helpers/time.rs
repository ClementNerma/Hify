use once_cell::sync::Lazy;
use time::{OffsetDateTime, UtcOffset};

// Required as the offset can fail to be get in some contexts
pub static OFFSET: Lazy<Option<UtcOffset>> =
    Lazy::new(|| UtcOffset::local_offset_at(OffsetDateTime::now_utc()).ok());

pub fn get_now() -> OffsetDateTime {
    OffsetDateTime::now_utc().to_offset(OFFSET.unwrap_or(UtcOffset::UTC))
}
