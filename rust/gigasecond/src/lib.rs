extern crate chrono;
use chrono::prelude::*;

pub fn after(datetime: chrono::DateTime<UTC>) -> chrono::DateTime<UTC> {
    let gigasec = 1_000_000_000;

    datetime + chrono::Duration::seconds(gigasec)
}
