//! _Fdate_ is a library for searching relative dates in English. It attempts to
//! be simple and unambigious yet provides a bit of flexibility with
//! configuration.

mod parser;
mod util;

use chrono::NaiveDate;
pub use parser::{FirstDay, Parser, ParserConfig};

/// Returns a date on successful parsing with defaults applied:
/// 1. Week starts with Monday,
/// 2. Calls for [chrono::Local::now],
/// 3. `next` and `last` mean the closest but today, e. g. `next Wednesday` will
///    mean tomorrow if today is Tuesday.
pub fn parse(input: &str) -> Option<NaiveDate> {
    Parser::new().parse(input)
}
