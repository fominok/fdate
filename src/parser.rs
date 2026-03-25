mod config;
mod syntax;

use chrono::{Datelike, Duration, Month, Months, NaiveDate, Weekday};
pub use config::{FirstDay, ParserConfig};
use syntax::{
    FdateExpression, IntervalUnit, RelativeDayOfMonth, RelativeDirection, RelativeInterval,
    RelativeIntervalDate, RelativeIntervalDateSpec, RelativePartialDate, RelativeWeekday,
};

use crate::util;

/// Configurable parser in case defaults don't suit.
#[derive(Default)]
pub struct Parser {
    config: ParserConfig,
}

impl Parser {
    /// Create [Parser] with default settings that will behave as [crate::parse]
    /// unless changed.
    pub fn new() -> Self {
        Default::default()
    }

    /// Create [Parser] with the provided configuration.
    pub fn with_config(config: ParserConfig) -> Self {
        Self { config }
    }

    /// Attempts to parse input into a date.
    pub fn parse(&self, input: &str) -> Option<NaiveDate> {
        Some(match FdateExpression::parse(input)? {
            FdateExpression::Absolute(naive_date) => naive_date,
            FdateExpression::RelativeInterval(RelativeInterval {
                direction,
                distance,
                unit,
            }) => self.apply_relative_interval(direction, distance, unit),
            FdateExpression::RelativeWeekday(RelativeWeekday { direction, weekday }) => {
                self.apply_relative_weekday(direction, weekday)
            }
            FdateExpression::RelativeDayOfMonth(RelativeDayOfMonth { direction, day }) => {
                self.apply_relative_day_of_month(direction, day)
            }
            FdateExpression::RelativePartialDate(RelativePartialDate {
                direction,
                day,
                month,
            }) => self.apply_relative_partial_date(direction, day, month)?,
            FdateExpression::RelativeIntervalDate(RelativeIntervalDate {
                direction,
                distance,
                target,
            }) => self.apply_relative_interval_date(direction, distance, target)?,
        })
    }

    fn apply_relative_interval(
        &self,
        direction: RelativeDirection,
        distance: u32,
        unit: IntervalUnit,
    ) -> NaiveDate {
        match direction {
            RelativeDirection::Future => match unit {
                IntervalUnit::Day => self.config.today + Duration::days(distance as i64),
                IntervalUnit::Week => self.config.today + Duration::weeks(distance as i64),
                IntervalUnit::Month => self
                    .config
                    .today
                    .checked_add_months(Months::new(distance))
                    .expect("out of time bounds"),
                IntervalUnit::Year => self
                    .config
                    .today
                    .checked_add_months(Months::new(distance * 12))
                    .expect("out of time bounds"),
            },
            RelativeDirection::Past => match unit {
                IntervalUnit::Day => self.config.today - Duration::days(distance as i64),
                IntervalUnit::Week => self.config.today - Duration::weeks(distance as i64),
                IntervalUnit::Month => self
                    .config
                    .today
                    .checked_sub_months(Months::new(distance))
                    .expect("out of time bounds"),
                IntervalUnit::Year => self
                    .config
                    .today
                    .checked_sub_months(Months::new(distance * 12))
                    .expect("out of time bounds"),
            },
        }
    }

    fn apply_relative_weekday(
        &self,
        direction: Option<RelativeDirection>,
        weekday: Weekday,
    ) -> NaiveDate {
        match direction {
            None => self.config.today + Duration::days(self.closest_future_weekday_offset(weekday)),
            Some(RelativeDirection::Future) => {
                if self.config.next_weekday_means_week {
                    self.weekday_in_relative_week(weekday, 1)
                } else {
                    self.config.today + Duration::days(self.closest_future_weekday_offset(weekday))
                }
            }
            Some(RelativeDirection::Past) => {
                if self.config.last_weekday_means_week {
                    self.weekday_in_relative_week(weekday, -1)
                } else {
                    self.config.today + Duration::days(-self.closest_past_weekday_offset(weekday))
                }
            }
        }
    }

    fn apply_relative_day_of_month(
        &self,
        direction: Option<RelativeDirection>,
        day: u32,
    ) -> NaiveDate {
        match direction {
            None => self.closest_future_day_of_month(day),
            Some(RelativeDirection::Future) => {
                if self.config.next_day_of_month_means_month {
                    self.day_in_relative_month(day, 1)
                } else {
                    self.closest_future_day_of_month(day)
                }
            }
            Some(RelativeDirection::Past) => {
                if self.config.last_day_of_month_means_month {
                    self.day_in_relative_month(day, -1)
                } else {
                    self.closest_past_day_of_month(day)
                }
            }
        }
    }

    fn apply_relative_partial_date(
        &self,
        direction: Option<RelativeDirection>,
        day: u32,
        month: Month,
    ) -> Option<NaiveDate> {
        match direction {
            None => self.closest_future_partial_date(day, month),
            Some(RelativeDirection::Future) => {
                if self.config.next_partial_date_means_year {
                    self.day_in_relative_year(day, month, 1)
                } else {
                    self.closest_future_partial_date(day, month)
                }
            }
            Some(RelativeDirection::Past) => {
                if self.config.last_partial_date_means_year {
                    self.day_in_relative_year(day, month, -1)
                } else {
                    self.closest_past_partial_date(day, month)
                }
            }
        }
    }

    fn apply_relative_interval_date(
        &self,
        direction: RelativeDirection,
        distance: u32,
        target: RelativeIntervalDateSpec,
    ) -> Option<NaiveDate> {
        match target {
            RelativeIntervalDateSpec::Weekday { weekday } => Some(match direction {
                RelativeDirection::Future => {
                    self.weekday_in_relative_week(weekday, distance as i64)
                }
                RelativeDirection::Past => {
                    self.weekday_in_relative_week(weekday, -(distance as i64))
                }
            }),
            RelativeIntervalDateSpec::DayOfMonth { day } => Some(match direction {
                RelativeDirection::Future => self.day_in_relative_month(day, distance as i32),
                RelativeDirection::Past => self.day_in_relative_month(day, -(distance as i32)),
            }),
            RelativeIntervalDateSpec::DayOfYear { day, month } => match direction {
                RelativeDirection::Future => self.day_in_relative_year(day, month, distance as i32),
                RelativeDirection::Past => {
                    self.day_in_relative_year(day, month, -(distance as i32))
                }
            },
        }
    }

    fn closest_future_weekday_offset(&self, weekday: Weekday) -> i64 {
        let current = self.weekday_index(self.config.today.weekday());
        let target = self.weekday_index(weekday);
        let delta = (target + 7 - current) % 7;

        if delta == 0 { 7 } else { delta as i64 }
    }

    fn closest_past_weekday_offset(&self, weekday: Weekday) -> i64 {
        let current = self.weekday_index(self.config.today.weekday());
        let target = self.weekday_index(weekday);
        let delta = (current + 7 - target) % 7;

        if delta == 0 { 7 } else { delta as i64 }
    }

    fn weekday_in_relative_week(&self, weekday: Weekday, week_offset: i64) -> NaiveDate {
        let start_of_current_week = self.config.today
            - Duration::days(self.weekday_index(self.config.today.weekday()) as i64);
        let start_of_target_week = start_of_current_week + Duration::weeks(week_offset);

        start_of_target_week + Duration::days(self.weekday_index(weekday) as i64)
    }

    fn weekday_index(&self, weekday: Weekday) -> u32 {
        match self.config.first_day {
            FirstDay::Monday => weekday.num_days_from_monday(),
            FirstDay::Sunday => weekday.num_days_from_sunday(),
        }
    }

    fn closest_future_day_of_month(&self, day: u32) -> NaiveDate {
        let target_day =
            util::from_ymd_clamp(self.config.today.year(), self.config.today.month(), day);
        if self.config.today < target_day {
            target_day
        } else {
            let updated_target = target_day + Months::new(1);
            util::from_ymd_clamp(updated_target.year(), updated_target.month(), day)
        }
    }

    fn closest_past_day_of_month(&self, day: u32) -> NaiveDate {
        let target_day =
            util::from_ymd_clamp(self.config.today.year(), self.config.today.month(), day);
        if self.config.today > target_day {
            target_day
        } else {
            let updated_target = target_day - Months::new(1);
            util::from_ymd_clamp(updated_target.year(), updated_target.month(), day)
        }
    }

    fn day_in_relative_month(&self, day: u32, month_offset: i32) -> NaiveDate {
        let target_month = if month_offset > 0 {
            self.config.today + Months::new(month_offset as u32)
        } else {
            self.config.today - Months::new(month_offset.unsigned_abs())
        };

        util::from_ymd_clamp(target_month.year(), target_month.month(), day)
    }

    fn closest_future_partial_date(&self, day: u32, month: Month) -> Option<NaiveDate> {
        let target_date =
            NaiveDate::from_ymd_opt(self.config.today.year(), month.number_from_month(), day)?;
        if target_date > self.config.today {
            Some(target_date)
        } else {
            target_date.with_year(self.config.today.year() + 1)
        }
    }

    fn closest_past_partial_date(&self, day: u32, month: Month) -> Option<NaiveDate> {
        let target_date =
            NaiveDate::from_ymd_opt(self.config.today.year(), month.number_from_month(), day)?;
        if target_date < self.config.today {
            Some(target_date)
        } else {
            target_date.with_year(self.config.today.year() - 1)
        }
    }

    fn day_in_relative_year(&self, day: u32, month: Month, year_offset: i32) -> Option<NaiveDate> {
        NaiveDate::from_ymd_opt(
            self.config.today.year() + year_offset,
            month.number_from_month(),
            day,
        )
    }
}

#[cfg(test)]
mod tests {
    use chrono::{NaiveDate, Weekday};

    use super::*;

    fn parser_with_today(today: NaiveDate) -> Parser {
        let mut config = ParserConfig::new();
        config.with_today(today);
        Parser::with_config(config)
    }

    #[test]
    fn resolves_weekdays_in_arbitrary_relative_weeks() {
        let mut config = ParserConfig::new();
        config
            .with_today(NaiveDate::from_ymd_opt(2026, 3, 24).unwrap())
            .week_starts_monday();
        let parser = Parser::with_config(config);

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
        let parser = parser_with_today(NaiveDate::from_ymd_opt(2025, 5, 13).unwrap());

        assert_eq!(
            parser.closest_future_day_of_month(14),
            NaiveDate::from_ymd_opt(2025, 5, 14).unwrap()
        );
    }

    #[test]
    fn resolves_closest_future_day_of_month_in_next_month_when_today_or_past() {
        let parser = parser_with_today(NaiveDate::from_ymd_opt(2025, 5, 14).unwrap());

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
        let parser = parser_with_today(NaiveDate::from_ymd_opt(2025, 4, 30).unwrap());

        assert_eq!(
            parser.closest_future_day_of_month(31),
            NaiveDate::from_ymd_opt(2025, 5, 31).unwrap()
        );

        let parser = parser_with_today(NaiveDate::from_ymd_opt(2025, 5, 31).unwrap());

        assert_eq!(
            parser.closest_future_day_of_month(31),
            NaiveDate::from_ymd_opt(2025, 6, 30).unwrap()
        );
    }

    #[test]
    fn resolves_closest_future_day_of_month_across_year_boundary() {
        let parser = parser_with_today(NaiveDate::from_ymd_opt(2025, 12, 31).unwrap());

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
        let parser = parser_with_today(NaiveDate::from_ymd_opt(2025, 5, 15).unwrap());

        assert_eq!(
            parser.closest_past_day_of_month(14),
            NaiveDate::from_ymd_opt(2025, 5, 14).unwrap()
        );
    }

    #[test]
    fn resolves_closest_past_day_of_month_in_previous_month_when_today_or_ahead() {
        let parser = parser_with_today(NaiveDate::from_ymd_opt(2025, 5, 14).unwrap());

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
        let parser = parser_with_today(NaiveDate::from_ymd_opt(2025, 7, 1).unwrap());

        assert_eq!(
            parser.closest_past_day_of_month(31),
            NaiveDate::from_ymd_opt(2025, 6, 30).unwrap()
        );

        let parser = parser_with_today(NaiveDate::from_ymd_opt(2025, 3, 30).unwrap());

        assert_eq!(
            parser.closest_past_day_of_month(31),
            NaiveDate::from_ymd_opt(2025, 2, 28).unwrap()
        );
    }

    #[test]
    fn resolves_closest_past_day_of_month_across_year_boundary() {
        let parser = parser_with_today(NaiveDate::from_ymd_opt(2026, 1, 1).unwrap());

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
