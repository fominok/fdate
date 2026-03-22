use chrono::Weekday;
use nom::{
    IResult, Parser, bytes::complete::tag_no_case, character::complete::multispace1,
    combinator::opt,
};

use crate::parse::util;

pub(crate) fn parse_relative_weekday(input: &str) -> IResult<&str, RelativeWeekday> {
    opt(RelativeWeekdayDirection::parse
        .and(multispace1)
        .map(|(direction, _)| direction))
    .and(util::parse_weekday)
    .map(|(direction, weekday)| RelativeWeekday { direction, weekday })
    .parse(input)
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct RelativeWeekday {
    pub direction: Option<RelativeWeekdayDirection>,
    pub weekday: Weekday,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum RelativeWeekdayDirection {
    Next,
    Last,
}

impl RelativeWeekdayDirection {
    pub fn parse(input: &str) -> IResult<&str, Self> {
        tag_no_case("next")
            .map(|_| Self::Next)
            .or(tag_no_case("last").map(|_| Self::Last))
            .parse(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_bare_relative_weekdays() {
        assert_eq!(
            parse_relative_weekday("monday"),
            Ok((
                "",
                RelativeWeekday {
                    direction: None,
                    weekday: Weekday::Mon,
                },
            ))
        );
        assert_eq!(
            parse_relative_weekday("thu"),
            Ok((
                "",
                RelativeWeekday {
                    direction: None,
                    weekday: Weekday::Thu,
                },
            ))
        );
    }

    #[test]
    fn parses_relative_weekdays_with_direction() {
        assert_eq!(
            parse_relative_weekday("next monday"),
            Ok((
                "",
                RelativeWeekday {
                    direction: Some(RelativeWeekdayDirection::Next),
                    weekday: Weekday::Mon,
                },
            ))
        );
        assert_eq!(
            parse_relative_weekday("last fri"),
            Ok((
                "",
                RelativeWeekday {
                    direction: Some(RelativeWeekdayDirection::Last),
                    weekday: Weekday::Fri,
                },
            ))
        );
    }

    #[test]
    fn parses_relative_weekdays_case_insensitively() {
        assert_eq!(
            parse_relative_weekday("Next Tuesday"),
            Ok((
                "",
                RelativeWeekday {
                    direction: Some(RelativeWeekdayDirection::Next),
                    weekday: Weekday::Tue,
                },
            ))
        );
        assert_eq!(
            parse_relative_weekday("LAST sunday"),
            Ok((
                "",
                RelativeWeekday {
                    direction: Some(RelativeWeekdayDirection::Last),
                    weekday: Weekday::Sun,
                },
            ))
        );
    }

    #[test]
    fn leaves_remaining_input_for_larger_parsers() {
        assert_eq!(
            parse_relative_weekday("next monday morning"),
            Ok((
                " morning",
                RelativeWeekday {
                    direction: Some(RelativeWeekdayDirection::Next),
                    weekday: Weekday::Mon,
                },
            ))
        );
    }

    #[test]
    fn rejects_invalid_relative_weekdays() {
        assert!(parse_relative_weekday("next").is_err());
        assert!(parse_relative_weekday("hello").is_err());
        assert!(parse_relative_weekday("nextmonday").is_err());
    }
}
