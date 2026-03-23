use chrono::{Month, NaiveDate};

pub(super) fn from_ymd_clamp(year: i32, month: u32, day: u32) -> NaiveDate {
    let num_days = Month::try_from(month as u8)
        .expect("must be a valid month")
        .num_days(year)
        .expect("must be a valid year");
    if num_days >= day as u8 {
        NaiveDate::from_ymd_opt(year, month as u32, day as u32).expect("must be a valid date")
    } else {
        NaiveDate::from_ymd_opt(year, month as u32, num_days as u32).expect("must be a valid date")
    }
}
