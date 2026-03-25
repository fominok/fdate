use chrono::{Month, Weekday};
use nom::{
    IResult, Parser, branch::alt, bytes::complete::tag_no_case, character::complete::multispace1,
    combinator::verify,
};

use super::{
    common::{IntervalUnit, RelativeDirection, parse_ordinal_day, parse_weekday},
    interval::{RelativeInterval, parse_explicit_non_literal_relative_interval},
    partial_date::parse_partial_date_body,
};

pub(super) fn parse_relative_interval_date(input: &str) -> IResult<&str, RelativeIntervalDate> {
    alt((
        parse_relative_week_interval_weekday,
        parse_relative_month_interval_day_of_month,
        parse_relative_year_interval_day_of_year,
    ))
    .parse(input)
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
    alt((
        tag_no_case("this week").map(|_| RelativeIntervalDate {
            direction: RelativeDirection::Future,
            distance: 0,
            target,
        }),
        tag_no_case("next week").map(|_| RelativeIntervalDate {
            direction: RelativeDirection::Future,
            distance: 1,
            target,
        }),
        tag_no_case("last week").map(|_| RelativeIntervalDate {
            direction: RelativeDirection::Past,
            distance: 1,
            target,
        }),
        verify(
            parse_explicit_non_literal_relative_interval,
            |RelativeInterval { unit, .. }| unit == &IntervalUnit::Week,
        )
        .map(|interval| RelativeIntervalDate {
            direction: interval.direction,
            distance: interval.distance,
            target,
        }),
    ))
    .parse(input)
}

fn parse_relative_month_interval_day_of_month(input: &str) -> IResult<&str, RelativeIntervalDate> {
    let (input, day) = parse_ordinal_day(input)?;
    let target = RelativeIntervalDateSpec::DayOfMonth { day };
    let (input, _) = multispace1(input)?;
    alt((
        tag_no_case("this month").map(|_| RelativeIntervalDate {
            direction: RelativeDirection::Future,
            distance: 0,
            target,
        }),
        tag_no_case("next month").map(|_| RelativeIntervalDate {
            direction: RelativeDirection::Future,
            distance: 1,
            target,
        }),
        tag_no_case("last month").map(|_| RelativeIntervalDate {
            direction: RelativeDirection::Past,
            distance: 1,
            target,
        }),
        verify(
            parse_explicit_non_literal_relative_interval,
            |RelativeInterval { unit, .. }| unit == &IntervalUnit::Month,
        )
        .map(|interval| RelativeIntervalDate {
            direction: interval.direction,
            distance: interval.distance,
            target,
        }),
    ))
    .parse(input)
}

fn parse_relative_year_interval_day_of_year(input: &str) -> IResult<&str, RelativeIntervalDate> {
    let (input, (day, month)) = parse_partial_date_body(input)?;
    let target = RelativeIntervalDateSpec::DayOfYear { day, month };
    let (input, _) = multispace1(input)?;
    alt((
        tag_no_case("this year").map(|_| RelativeIntervalDate {
            direction: RelativeDirection::Future,
            distance: 0,
            target,
        }),
        tag_no_case("next year").map(|_| RelativeIntervalDate {
            direction: RelativeDirection::Future,
            distance: 1,
            target,
        }),
        tag_no_case("last year").map(|_| RelativeIntervalDate {
            direction: RelativeDirection::Past,
            distance: 1,
            target,
        }),
        verify(
            parse_explicit_non_literal_relative_interval,
            |RelativeInterval { unit, .. }| unit == &IntervalUnit::Year,
        )
        .map(|interval| RelativeIntervalDate {
            direction: interval.direction,
            distance: interval.distance,
            target,
        }),
    ))
    .parse(input)
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

    #[test]
    fn parses_this_month_days() {
        assert_eq!(
            parse_relative_month_interval_day_of_month("14th this month"),
            Ok((
                "",
                RelativeIntervalDate {
                    direction: RelativeDirection::Future,
                    distance: 0,
                    target: RelativeIntervalDateSpec::DayOfMonth { day: 14 },
                },
            ))
        );
    }

    #[test]
    fn parses_relative_month_intervals() {
        assert_eq!(
            parse_relative_month_interval_day_of_month("17 in 2 months"),
            Ok((
                "",
                RelativeIntervalDate {
                    direction: RelativeDirection::Future,
                    distance: 2,
                    target: RelativeIntervalDateSpec::DayOfMonth { day: 17 },
                },
            ))
        );
        assert_eq!(
            parse_relative_month_interval_day_of_month("10 a month ago"),
            Ok((
                "",
                RelativeIntervalDate {
                    direction: RelativeDirection::Past,
                    distance: 1,
                    target: RelativeIntervalDateSpec::DayOfMonth { day: 10 },
                },
            ))
        );
    }

    #[test]
    fn rejects_non_month_intervals() {
        assert!(parse_relative_month_interval_day_of_month("14th this week").is_err());
        assert!(parse_relative_month_interval_day_of_month("hello this month").is_err());
    }

    #[test]
    fn parses_this_year_days() {
        assert_eq!(
            parse_relative_year_interval_day_of_year("1st may this year"),
            Ok((
                "",
                RelativeIntervalDate {
                    direction: RelativeDirection::Future,
                    distance: 0,
                    target: RelativeIntervalDateSpec::DayOfYear {
                        day: 1,
                        month: Month::May,
                    },
                },
            ))
        );
    }

    #[test]
    fn parses_relative_year_intervals() {
        assert_eq!(
            parse_relative_year_interval_day_of_year("10th july in 2 years"),
            Ok((
                "",
                RelativeIntervalDate {
                    direction: RelativeDirection::Future,
                    distance: 2,
                    target: RelativeIntervalDateSpec::DayOfYear {
                        day: 10,
                        month: Month::July,
                    },
                },
            ))
        );
        assert_eq!(
            parse_relative_year_interval_day_of_year("may 1 a year ago"),
            Ok((
                "",
                RelativeIntervalDate {
                    direction: RelativeDirection::Past,
                    distance: 1,
                    target: RelativeIntervalDateSpec::DayOfYear {
                        day: 1,
                        month: Month::May,
                    },
                },
            ))
        );
    }

    #[test]
    fn rejects_non_year_intervals() {
        assert!(parse_relative_year_interval_day_of_year("1st may this month").is_err());
        assert!(parse_relative_year_interval_day_of_year("14th this year").is_err());
    }

    #[test]
    fn parses_relative_interval_dates() {
        assert!(parse_relative_interval_date("monday this week").is_ok());
        assert!(parse_relative_interval_date("14th this month").is_ok());
        assert!(parse_relative_interval_date("1st may this year").is_ok());
    }
}
