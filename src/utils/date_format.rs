use chrono::{NaiveDateTime, FixedOffset, TimeZone};
use serde::{self, Serializer};

pub fn serialize<S>(datetime: &NaiveDateTime, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    // Define UTC+1 offset (West Africa Time, WAT)
    let offset = FixedOffset::east(3600); // 3600 seconds = 1 hour

    // Convert NaiveDateTime (UTC) to DateTime<Utc> and then to UTC+1
    let datetime_wat = offset.from_utc_datetime(datetime);

    // Format as "dd-mm-yyyy h:m"
    let formatted = datetime_wat.format("%d-%m-%Y %H:%M").to_string(); // Example: "26-05-2024 20:18"

    serializer.serialize_str(&formatted)
}
