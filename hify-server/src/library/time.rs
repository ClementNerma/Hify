use once_cell::sync::Lazy;
use time::{OffsetDateTime, UtcOffset};

// Required as the offset can fail to be get in some contexts
static OFFSET: Lazy<UtcOffset> = Lazy::new(|| {
    UtcOffset::local_offset_at(OffsetDateTime::now_utc()).unwrap_or_else(|_| {
        eprintln!("Failed to determine local offset, UTC will be used instead");
        UtcOffset::UTC
    })
});

pub fn get_now() -> OffsetDateTime {
    OffsetDateTime::now_utc().to_offset(*OFFSET)
}
