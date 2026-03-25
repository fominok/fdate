use chrono::NaiveDate;
pub(crate) use common::{IntervalUnit, RelativeDirection};
pub(crate) use day_of_month::RelativeDayOfMonth;
pub(crate) use interval::RelativeInterval;
use nom::{Parser, branch::alt, combinator::all_consuming};
pub(crate) use partial_date::RelativePartialDate;
pub(crate) use relative_interval_date::{RelativeIntervalDate, RelativeIntervalDateSpec};
pub(crate) use relative_weekday::RelativeWeekday;

mod absolute;
mod common;
mod day_of_month;
mod interval;
mod partial_date;
mod relative_interval_date;
mod relative_weekday;

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum FdateExpression {
    Absolute(NaiveDate),
    RelativeInterval(RelativeInterval),
    RelativeWeekday(RelativeWeekday),
    RelativeDayOfMonth(RelativeDayOfMonth),
    RelativePartialDate(RelativePartialDate),
    RelativeIntervalDate(RelativeIntervalDate),
}

impl FdateExpression {
    pub(crate) fn parse(input: &str) -> Option<Self> {
        all_consuming(alt((
            absolute::parse_absolute.map(Self::Absolute),
            interval::parse_relative_interval.map(Self::RelativeInterval),
            relative_interval_date::parse_relative_interval_date.map(Self::RelativeIntervalDate),
            relative_weekday::parse_relative_weekday.map(Self::RelativeWeekday),
            partial_date::parse_relative_partial_date.map(Self::RelativePartialDate),
            day_of_month::parse_relative_day_of_month.map(Self::RelativeDayOfMonth),
        )))
        .parse(input)
        .map(|(_, out)| out)
        .ok()
    }
}

#[cfg(test)]
mod tests {
    use chrono::{Month, Weekday};

    use super::*;

    #[test]
    fn parses_absolute_expressions() {
        assert_eq!(
            FdateExpression::parse("2000-10-10"),
            Some(FdateExpression::Absolute(
                NaiveDate::from_ymd_opt(2000, 10, 10).unwrap()
            ))
        );
    }

    #[test]
    fn parses_relative_interval_expressions() {
        assert_eq!(
            FdateExpression::parse("a week ago"),
            Some(FdateExpression::RelativeInterval(RelativeInterval {
                direction: RelativeDirection::Past,
                distance: 1,
                unit: IntervalUnit::Week,
            }))
        );
    }

    #[test]
    fn parses_relative_weekday_expressions() {
        assert_eq!(
            FdateExpression::parse("next monday"),
            Some(FdateExpression::RelativeWeekday(RelativeWeekday {
                direction: Some(RelativeDirection::Future),
                weekday: Weekday::Mon,
            }))
        );
    }

    #[test]
    fn parses_relative_day_of_month_expressions() {
        assert_eq!(
            FdateExpression::parse("last april 14"),
            Some(FdateExpression::RelativePartialDate(RelativePartialDate {
                direction: Some(RelativeDirection::Past),
                day: 14,
                month: Month::April,
            }))
        );
    }

    #[test]
    fn parses_relative_interval_date_expressions() {
        assert_eq!(
            FdateExpression::parse("monday this week"),
            Some(FdateExpression::RelativeIntervalDate(
                RelativeIntervalDate {
                    direction: RelativeDirection::Future,
                    distance: 0,
                    target: RelativeIntervalDateSpec::Weekday {
                        weekday: Weekday::Mon,
                    },
                }
            ))
        );
    }

    #[test]
    fn rejects_inputs_with_trailing_text() {
        assert_eq!(FdateExpression::parse("today please"), None);
        assert_eq!(FdateExpression::parse("monday this week please"), None);
    }

    #[test]
    fn rejects_invalid_inputs() {
        assert_eq!(FdateExpression::parse("hello"), None);
        assert_eq!(FdateExpression::parse("next"), None);
    }
}
