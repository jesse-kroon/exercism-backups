use std::ops::Add;

use time::{Duration, PrimitiveDateTime as DateTime};

const GIGASECOND: u64 = 1_000_000_000;

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    start.add(Duration::seconds(GIGASECOND as i64))
}