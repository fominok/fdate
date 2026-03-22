use chrono::{Month, Weekday};
use nom::{
    IResult, Parser, bytes::complete::tag_no_case, character::complete::multispace1,
    combinator::verify,
};

use crate::parse::{
    common::{IntervalUnit, RelativeDirection, parse_weekday},
    interval::{RelativeInterval, parse_non_literal_relative_interval},
};

pub(crate) fn parse_relative_interval_date(input: &str) -> IResult<&str, RelativeIntervalDate> {
    todo!()
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct RelativeIntervalDate {
    pub direction: RelativeDirection,
    pub distance: u32,
    pub target: RelativeIntervalDateSpec,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum RelativeIntervalDateSpec {
    Weekday { weekday: Weekday },
    DayOfMonth { day: u32 },
    DayOfYear { day: u32, month: Month },
}

fn parse_relative_week_interval_weekday(input: &str) -> IResult<&str, RelativeIntervalDate> {
    let (input, weekday) = parse_weekday(input)?;
    let target = RelativeIntervalDateSpec::Weekday { weekday };
    let (input, _) = multispace1(input)?;
    tag_no_case("this week")
        .map(|_| RelativeIntervalDate {
            direction: RelativeDirection::Future,
            distance: 0,
            target,
        })
        .or(verify(
            parse_non_literal_relative_interval,
            |RelativeInterval { unit, .. }| unit == &IntervalUnit::Week,
        )
        .map(|interval| RelativeIntervalDate {
            direction: interval.direction,
            distance: interval.value,
            target,
        }))
        .parse(input)
}

fn parse_relative_month_interval_day_of_month(input: &str) -> IResult<&str, RelativeIntervalDate> {
    todo!()
}

fn parse_relative_year_interval_day_of_year(input: &str) -> IResult<&str, RelativeIntervalDate> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_this_week_weekdays() {
        assert_eq!(
            parse_relative_week_interval_weekday("monday this week"),
            Ok((
                "",
                RelativeIntervalDate {
                    direction: RelativeDirection::Future,
                    distance: 0,
                    target: RelativeIntervalDateSpec::Weekday {
                        weekday: Weekday::Mon,
                    },
                },
            ))
        );
        assert_eq!(
            parse_relative_week_interval_weekday("Thu this week"),
            Ok((
                "",
                RelativeIntervalDate {
                    direction: RelativeDirection::Future,
                    distance: 0,
                    target: RelativeIntervalDateSpec::Weekday {
                        weekday: Weekday::Thu,
                    },
                },
            ))
        );
    }

    #[test]
    fn parses_relative_week_intervals() {
        assert_eq!(
            parse_relative_week_interval_weekday("friday in 2 weeks"),
            Ok((
                "",
                RelativeIntervalDate {
                    direction: RelativeDirection::Future,
                    distance: 2,
                    target: RelativeIntervalDateSpec::Weekday {
                        weekday: Weekday::Fri,
                    },
                },
            ))
        );
        assert_eq!(
            parse_relative_week_interval_weekday("tuesday a week ago"),
            Ok((
                "",
                RelativeIntervalDate {
                    direction: RelativeDirection::Past,
                    distance: 1,
                    target: RelativeIntervalDateSpec::Weekday {
                        weekday: Weekday::Tue,
                    },
                },
            ))
        );
    }

    #[test]
    fn leaves_remaining_input_for_larger_parsers() {
        assert_eq!(
            parse_relative_week_interval_weekday("wednesday this week please"),
            Ok((
                " please",
                RelativeIntervalDate {
                    direction: RelativeDirection::Future,
                    distance: 0,
                    target: RelativeIntervalDateSpec::Weekday {
                        weekday: Weekday::Wed,
                    },
                },
            ))
        );
    }

    #[test]
    fn rejects_non_week_intervals() {
        assert!(parse_relative_week_interval_weekday("monday in 2 months").is_err());
        assert!(parse_relative_week_interval_weekday("monday today").is_err());
        assert!(parse_relative_week_interval_weekday("hello this week").is_err());
    }
}
