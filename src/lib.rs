//! _Fdate_ is a library for searching relative dates in English. It attempts to
//! be simple and unambigious yet provides a bit of flexibility with
//! configuration.

mod parse;
mod util;

use chrono::{Datelike, Duration, Local, Month, Months, NaiveDate, Weekday};

use crate::parse::{
    FdateExpression, IntervalUnit, RelativeDayOfMonth, RelativeDirection, RelativeInterval,
    RelativeIntervalDate, RelativeIntervalDateSpec, RelativePartialDate, RelativeWeekday,
};

/// Returns a date on successful parsing with defaults applied:
/// 1. Week starts with Monday,
/// 2. Calls for [chrono::Local::now],
/// 3. `next` and `last` mean the closest but today, e. g. `next Wednesday` will
///    mean tomorrow if today is Tuesday.
pub fn parse(input: &str) -> Option<NaiveDate> {
    Parser::new().parse(input)
}

/// Configurable parser in case defaults don't suit.
pub struct Parser {
    first_day: FirstDay,
    next_weekday_means_week: bool,
    next_day_of_month_means_month: bool,
    next_partial_date_means_year: bool,
    last_weekday_means_week: bool,
    last_day_of_month_means_month: bool,
    last_partial_date_means_year: bool,
    today: NaiveDate,
}

impl Default for Parser {
    fn default() -> Self {
        Self {
            first_day: FirstDay::Monday,
            next_weekday_means_week: false,
            next_day_of_month_means_month: false,
            next_partial_date_means_year: false,
            last_weekday_means_week: false,
            last_day_of_month_means_month: false,
            last_partial_date_means_year: false,
            today: Local::now().date_naive(),
        }
    }
}

impl Parser {
    /// Create [Parser] with default settings that will behave as [parse] unless
    /// changed.
    pub fn new() -> Self {
        Self::default()
    }

    /// Attempts to parse input into a date.
    pub fn parse(&self, input: &str) -> Option<NaiveDate> {
        Some(match FdateExpression::parse(input)? {
            FdateExpression::Absolute(naive_date) => naive_date,
            FdateExpression::RelativeInterval(RelativeInterval {
                direction,
                distance,
                unit,
            }) => match direction {
                RelativeDirection::Future => match unit {
                    IntervalUnit::Day => self.today + Duration::days(distance as i64),
                    IntervalUnit::Week => self.today + Duration::weeks(distance as i64),
                    IntervalUnit::Month => self
                        .today
                        .checked_add_months(Months::new(distance))
                        .expect("out of time bounds"),
                    IntervalUnit::Year => self
                        .today
                        .checked_add_months(Months::new(distance * 12))
                        .expect("out of time bounds"),
                },
                RelativeDirection::Past => match unit {
                    IntervalUnit::Day => self.today - Duration::days(distance as i64),
                    IntervalUnit::Week => self.today - Duration::weeks(distance as i64),
                    IntervalUnit::Month => self
                        .today
                        .checked_sub_months(Months::new(distance))
                        .expect("out of time bounds"),
                    IntervalUnit::Year => self
                        .today
                        .checked_sub_months(Months::new(distance * 12))
                        .expect("out of time bounds"),
                },
            },
            FdateExpression::RelativeWeekday(RelativeWeekday { direction, weekday }) => {
                match direction {
                    Option::None => {
                        self.today + Duration::days(self.closest_future_weekday_offset(weekday))
                    }
                    Some(RelativeDirection::Future) => {
                        if self.next_weekday_means_week {
                            self.weekday_in_relative_week(weekday, 1)
                        } else {
                            self.today + Duration::days(self.closest_future_weekday_offset(weekday))
                        }
                    }
                    Some(RelativeDirection::Past) => {
                        if self.last_weekday_means_week {
                            self.weekday_in_relative_week(weekday, -1)
                        } else {
                            self.today + Duration::days(-self.closest_past_weekday_offset(weekday))
                        }
                    }
                }
            }
            FdateExpression::RelativeDayOfMonth(RelativeDayOfMonth { direction, day }) => {
                match direction {
                    Option::None => self.closest_future_day_of_month(day),
                    Some(RelativeDirection::Future) => {
                        if self.next_day_of_month_means_month {
                            self.day_in_relative_month(day, 1)
                        } else {
                            self.closest_future_day_of_month(day)
                        }
                    }
                    Some(RelativeDirection::Past) => {
                        if self.last_day_of_month_means_month {
                            self.day_in_relative_month(day, -1)
                        } else {
                            self.closest_past_day_of_month(day)
                        }
                    }
                }
            }
            FdateExpression::RelativePartialDate(RelativePartialDate {
                direction,
                day,
                month,
            }) => match direction {
                Option::None => self.closest_future_partial_date(day, month)?,
                Some(RelativeDirection::Future) => {
                    if self.next_partial_date_means_year {
                        self.day_in_relative_year(day, month, 1)?
                    } else {
                        self.closest_future_partial_date(day, month)?
                    }
                }
                Some(RelativeDirection::Past) => {
                    if self.last_partial_date_means_year {
                        self.day_in_relative_year(day, month, -1)?
                    } else {
                        self.closest_past_partial_date(day, month)?
                    }
                }
            },
            FdateExpression::RelativeIntervalDate(RelativeIntervalDate {
                direction,
                distance,
                target: RelativeIntervalDateSpec::Weekday { weekday },
            }) => match direction {
                RelativeDirection::Future => {
                    self.weekday_in_relative_week(weekday, distance as i64)
                }
                RelativeDirection::Past => {
                    self.weekday_in_relative_week(weekday, -(distance as i64))
                }
            },
            FdateExpression::RelativeIntervalDate(RelativeIntervalDate {
                direction,
                distance,
                target: RelativeIntervalDateSpec::DayOfMonth { day },
            }) => match direction {
                RelativeDirection::Future => self.day_in_relative_month(day, distance as i32),
                RelativeDirection::Past => self.day_in_relative_month(day, -(distance as i32)),
            },
            FdateExpression::RelativeIntervalDate(RelativeIntervalDate {
                direction,
                distance,
                target: RelativeIntervalDateSpec::DayOfYear { day, month },
            }) => match direction {
                RelativeDirection::Future => {
                    self.day_in_relative_year(day, month, distance as i32)?
                }
                RelativeDirection::Past => {
                    self.day_in_relative_year(day, month, -(distance as i32))?
                }
            },
        })
    }

    /// Set today's date explicitly
    pub fn with_today(&mut self, today: NaiveDate) -> &mut Self {
        self.today = today;
        self
    }

    /// First day of week is Monday
    pub fn week_starts_monday(&mut self) -> &mut Self {
        self.first_day = FirstDay::Monday;
        self
    }

    /// First day of week is Sunday
    pub fn week_starts_sunday(&mut self) -> &mut Self {
        self.first_day = FirstDay::Sunday;
        self
    }

    /// `next Wednesday` will be in 8 days if today is Tuesday
    pub fn next_weekday_means_week(&mut self) -> &mut Self {
        self.next_weekday_means_week = true;
        self
    }

    /// `next Wednesday` will be tomorrow if today is Tuesday (default)
    pub fn next_weekday_means_closest(&mut self) -> &mut Self {
        self.next_weekday_means_week = false;
        self
    }

    /// `next 14th` will be next month if today is 13th (in ~31 days)
    pub fn next_day_of_month_means_month(&mut self) -> &mut Self {
        self.next_day_of_month_means_month = true;
        self
    }

    /// `next 14th` will be tomorrow if today is 13th (default)
    pub fn next_day_of_month_means_closest(&mut self) -> &mut Self {
        self.next_day_of_month_means_month = false;
        self
    }

    /// `next April 14th` will be in 2027 if it's 2026 now
    pub fn next_partial_date_means_year(&mut self) -> &mut Self {
        self.next_partial_date_means_year = true;
        self
    }

    /// `next April 14th` will be in one month if it's March now (default).
    pub fn next_partial_date_means_closest(&mut self) -> &mut Self {
        self.next_partial_date_means_year = false;
        self
    }

    /// `last Wednesday` was 8 days ago if today is Thursday
    pub fn last_weekday_means_week(&mut self) -> &mut Self {
        self.last_weekday_means_week = true;
        self
    }

    /// `last Wednesday` was yesterday if today is Thursday (default)
    pub fn last_weekday_means_closest(&mut self) -> &mut Self {
        self.last_weekday_means_week = false;
        self
    }

    /// `last 14th` was in the previous month if today is 15th (in ~31 days)
    pub fn last_day_of_month_means_month(&mut self) -> &mut Self {
        self.last_day_of_month_means_month = true;
        self
    }

    /// `last 14th` was yesterday if today is 15th (default)
    pub fn last_day_of_month_means_closest(&mut self) -> &mut Self {
        self.last_day_of_month_means_month = false;
        self
    }

    /// `last April 14th` was in 2025 if it's 2026 now
    pub fn last_partial_date_means_year(&mut self) -> &mut Self {
        self.last_partial_date_means_year = true;
        self
    }

    /// `next April 14th` was in the previous month if it's May now (default).
    pub fn last_partial_date_means_closest(&mut self) -> &mut Self {
        self.last_partial_date_means_year = false;
        self
    }

    fn closest_future_weekday_offset(&self, weekday: Weekday) -> i64 {
        let current = self.weekday_index(self.today.weekday());
        let target = self.weekday_index(weekday);
        let delta = (target + 7 - current) % 7;

        if delta == 0 { 7 } else { delta as i64 }
    }

    fn closest_past_weekday_offset(&self, weekday: Weekday) -> i64 {
        let current = self.weekday_index(self.today.weekday());
        let target = self.weekday_index(weekday);
        let delta = (current + 7 - target) % 7;

        if delta == 0 { 7 } else { delta as i64 }
    }

    fn weekday_in_relative_week(&self, weekday: Weekday, week_offset: i64) -> NaiveDate {
        let start_of_current_week =
            self.today - Duration::days(self.weekday_index(self.today.weekday()) as i64);
        let start_of_target_week = start_of_current_week + Duration::weeks(week_offset);

        start_of_target_week + Duration::days(self.weekday_index(weekday) as i64)
    }

    fn weekday_index(&self, weekday: Weekday) -> u32 {
        match self.first_day {
            FirstDay::Monday => weekday.num_days_from_monday(),
            FirstDay::Sunday => weekday.num_days_from_sunday(),
        }
    }

    fn closest_future_day_of_month(&self, day: u32) -> NaiveDate {
        let target_day = util::from_ymd_clamp(self.today.year(), self.today.month(), day);
        if self.today < target_day {
            target_day
        } else {
            let updated_target = target_day + Months::new(1);
            util::from_ymd_clamp(updated_target.year(), updated_target.month(), day)
        }
    }

    fn closest_past_day_of_month(&self, day: u32) -> NaiveDate {
        let target_day = util::from_ymd_clamp(self.today.year(), self.today.month(), day);
        if self.today > target_day {
            target_day
        } else {
            let updated_target = target_day - Months::new(1);
            util::from_ymd_clamp(updated_target.year(), updated_target.month(), day)
        }
    }

    fn day_in_relative_month(&self, day: u32, month_offset: i32) -> NaiveDate {
        let target_month = if month_offset > 0 {
            self.today + Months::new(month_offset as u32)
        } else {
            self.today - Months::new(month_offset.abs() as u32)
        };

        util::from_ymd_clamp(target_month.year(), target_month.month(), day)
    }

    fn closest_future_partial_date(&self, day: u32, month: Month) -> Option<NaiveDate> {
        let target_date =
            NaiveDate::from_ymd_opt(self.today.year(), month.number_from_month(), day)?;
        if target_date > self.today {
            Some(target_date)
        } else {
            target_date.with_year(self.today.year() + 1)
        }
    }

    fn closest_past_partial_date(&self, day: u32, month: Month) -> Option<NaiveDate> {
        let target_date =
            NaiveDate::from_ymd_opt(self.today.year(), month.number_from_month(), day)?;
        if target_date < self.today {
            Some(target_date)
        } else {
            target_date.with_year(self.today.year() - 1)
        }
    }

    fn day_in_relative_year(&self, day: u32, month: Month, year_offset: i32) -> Option<NaiveDate> {
        NaiveDate::from_ymd_opt(
            self.today.year() + year_offset,
            month.number_from_month(),
            day,
        )
    }
}

enum FirstDay {
    Monday,
    Sunday,
}

#[cfg(test)]
mod tests {
    use chrono::{NaiveDate, Weekday};

    use super::*;

    #[test]
    fn resolves_weekdays_in_arbitrary_relative_weeks() {
        let mut parser = Parser::new();
        parser
            .with_today(NaiveDate::from_ymd_opt(2026, 3, 24).unwrap())
            .week_starts_monday();

        assert_eq!(
            parser.weekday_in_relative_week(Weekday::Wed, 0),
            NaiveDate::from_ymd_opt(2026, 3, 25).unwrap()
        );
        assert_eq!(
            parser.weekday_in_relative_week(Weekday::Wed, 3),
            NaiveDate::from_ymd_opt(2026, 4, 15).unwrap()
        );
        assert_eq!(
            parser.weekday_in_relative_week(Weekday::Mon, -2),
            NaiveDate::from_ymd_opt(2026, 3, 9).unwrap()
        );
    }

    #[test]
    fn resolves_closest_future_day_of_month_in_current_month_when_still_ahead() {
        let mut parser = Parser::new();
        parser.with_today(NaiveDate::from_ymd_opt(2025, 5, 13).unwrap());

        assert_eq!(
            parser.closest_future_day_of_month(14),
            NaiveDate::from_ymd_opt(2025, 5, 14).unwrap()
        );
    }

    #[test]
    fn resolves_closest_future_day_of_month_in_next_month_when_today_or_past() {
        let mut parser = Parser::new();
        parser.with_today(NaiveDate::from_ymd_opt(2025, 5, 14).unwrap());

        assert_eq!(
            parser.closest_future_day_of_month(14),
            NaiveDate::from_ymd_opt(2025, 6, 14).unwrap()
        );
        assert_eq!(
            parser.closest_future_day_of_month(13),
            NaiveDate::from_ymd_opt(2025, 6, 13).unwrap()
        );
    }

    #[test]
    fn clamps_closest_future_day_of_month_to_end_of_next_month_when_needed() {
        let mut parser = Parser::new();
        parser.with_today(NaiveDate::from_ymd_opt(2025, 4, 30).unwrap());

        assert_eq!(
            parser.closest_future_day_of_month(31),
            NaiveDate::from_ymd_opt(2025, 5, 31).unwrap()
        );

        parser.with_today(NaiveDate::from_ymd_opt(2025, 5, 31).unwrap());

        assert_eq!(
            parser.closest_future_day_of_month(31),
            NaiveDate::from_ymd_opt(2025, 6, 30).unwrap()
        );
    }

    #[test]
    fn resolves_closest_future_day_of_month_across_year_boundary() {
        let mut parser = Parser::new();
        parser.with_today(NaiveDate::from_ymd_opt(2025, 12, 31).unwrap());

        assert_eq!(
            parser.closest_future_day_of_month(1),
            NaiveDate::from_ymd_opt(2026, 1, 1).unwrap()
        );
        assert_eq!(
            parser.closest_future_day_of_month(31),
            NaiveDate::from_ymd_opt(2026, 1, 31).unwrap()
        );
    }

    #[test]
    fn resolves_closest_past_day_of_month_in_current_month_when_still_behind() {
        let mut parser = Parser::new();
        parser.with_today(NaiveDate::from_ymd_opt(2025, 5, 15).unwrap());

        assert_eq!(
            parser.closest_past_day_of_month(14),
            NaiveDate::from_ymd_opt(2025, 5, 14).unwrap()
        );
    }

    #[test]
    fn resolves_closest_past_day_of_month_in_previous_month_when_today_or_ahead() {
        let mut parser = Parser::new();
        parser.with_today(NaiveDate::from_ymd_opt(2025, 5, 14).unwrap());

        assert_eq!(
            parser.closest_past_day_of_month(14),
            NaiveDate::from_ymd_opt(2025, 4, 14).unwrap()
        );
        assert_eq!(
            parser.closest_past_day_of_month(15),
            NaiveDate::from_ymd_opt(2025, 4, 15).unwrap()
        );
    }

    #[test]
    fn clamps_closest_past_day_of_month_to_end_of_previous_month_when_needed() {
        let mut parser = Parser::new();
        parser.with_today(NaiveDate::from_ymd_opt(2025, 7, 1).unwrap());

        assert_eq!(
            parser.closest_past_day_of_month(31),
            NaiveDate::from_ymd_opt(2025, 6, 30).unwrap()
        );

        parser.with_today(NaiveDate::from_ymd_opt(2025, 3, 30).unwrap());

        assert_eq!(
            parser.closest_past_day_of_month(31),
            NaiveDate::from_ymd_opt(2025, 2, 28).unwrap()
        );
    }

    #[test]
    fn resolves_closest_past_day_of_month_across_year_boundary() {
        let mut parser = Parser::new();
        parser.with_today(NaiveDate::from_ymd_opt(2026, 1, 1).unwrap());

        assert_eq!(
            parser.closest_past_day_of_month(31),
            NaiveDate::from_ymd_opt(2025, 12, 31).unwrap()
        );
        assert_eq!(
            parser.closest_past_day_of_month(1),
            NaiveDate::from_ymd_opt(2025, 12, 1).unwrap()
        );
    }
}
