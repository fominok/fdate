use chrono::Month;
use nom::{IResult, Parser, character::complete::multispace1, combinator::opt};

use crate::parse::common::{self, RelativeDirection};

pub(crate) fn parse_relative_day_of_month(input: &str) -> IResult<&str, RelativeDayOfMonth> {
    opt(common::parse_relative_direction
        .and(multispace1)
        .map(|(direction, _)| direction))
    .and(parse_day_of_month_components)
    .map(|(direction, (day, month))| RelativeDayOfMonth {
        direction,
        day,
        month,
    })
    .parse(input)
}

fn parse_day_of_month_components(input: &str) -> IResult<&str, (u32, Option<Month>)> {
    (common::parse_ordinal_day.and(opt(multispace1
        .and(common::parse_month)
        .map(|(_, month)| month))))
    .map(|(day, month)| (day, month))
    .or(
        (common::parse_month, multispace1, common::parse_ordinal_day)
            .map(|(month, _, day)| (day, Some(month))),
    )
    .parse(input)
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct RelativeDayOfMonth {
    pub direction: Option<RelativeDirection>,
    pub day: u32,
    pub month: Option<Month>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_bare_relative_days_of_month() {
        assert_eq!(
            parse_relative_day_of_month("14"),
            Ok((
                "",
                RelativeDayOfMonth {
                    direction: None,
                    day: 14,
                    month: None,
                },
            ))
        );
        assert_eq!(
            parse_relative_day_of_month("14th"),
            Ok((
                "",
                RelativeDayOfMonth {
                    direction: None,
                    day: 14,
                    month: None,
                },
            ))
        );
    }

    #[test]
    fn parses_relative_days_of_month_with_direction() {
        assert_eq!(
            parse_relative_day_of_month("next 14th"),
            Ok((
                "",
                RelativeDayOfMonth {
                    direction: Some(RelativeDirection::Future),
                    day: 14,
                    month: None,
                },
            ))
        );
        assert_eq!(
            parse_relative_day_of_month("last 14"),
            Ok((
                "",
                RelativeDayOfMonth {
                    direction: Some(RelativeDirection::Past),
                    day: 14,
                    month: None,
                },
            ))
        );
    }

    #[test]
    fn parses_relative_days_of_month_case_insensitively() {
        assert_eq!(
            parse_relative_day_of_month("Next 1st"),
            Ok((
                "",
                RelativeDayOfMonth {
                    direction: Some(RelativeDirection::Future),
                    day: 1,
                    month: None,
                },
            ))
        );
        assert_eq!(
            parse_relative_day_of_month("LAST 22ND"),
            Ok((
                "",
                RelativeDayOfMonth {
                    direction: Some(RelativeDirection::Past),
                    day: 22,
                    month: None,
                },
            ))
        );
    }

    #[test]
    fn parses_relative_days_of_month_with_actual_month() {
        assert_eq!(
            parse_relative_day_of_month("14th april"),
            Ok((
                "",
                RelativeDayOfMonth {
                    direction: None,
                    day: 14,
                    month: Some(Month::April),
                },
            ))
        );
        assert_eq!(
            parse_relative_day_of_month("next 1st December"),
            Ok((
                "",
                RelativeDayOfMonth {
                    direction: Some(RelativeDirection::Future),
                    day: 1,
                    month: Some(Month::December),
                },
            ))
        );
        assert_eq!(
            parse_relative_day_of_month("next April 14th"),
            Ok((
                "",
                RelativeDayOfMonth {
                    direction: Some(RelativeDirection::Future),
                    day: 14,
                    month: Some(Month::April),
                },
            ))
        );
        assert_eq!(
            parse_relative_day_of_month("last 14 April"),
            Ok((
                "",
                RelativeDayOfMonth {
                    direction: Some(RelativeDirection::Past),
                    day: 14,
                    month: Some(Month::April),
                },
            ))
        );
        assert_eq!(
            parse_relative_day_of_month("last April 14"),
            Ok((
                "",
                RelativeDayOfMonth {
                    direction: Some(RelativeDirection::Past),
                    day: 14,
                    month: Some(Month::April),
                },
            ))
        );
    }

    #[test]
    fn leaves_remaining_input_for_larger_parsers() {
        assert_eq!(
            parse_relative_day_of_month("14th april please"),
            Ok((
                " please",
                RelativeDayOfMonth {
                    direction: None,
                    day: 14,
                    month: Some(Month::April),
                },
            ))
        );
    }

    #[test]
    fn rejects_invalid_relative_days_of_month() {
        assert!(parse_relative_day_of_month("next").is_err());
        assert!(parse_relative_day_of_month("0th").is_err());
        assert!(parse_relative_day_of_month("32").is_err());
        assert!(parse_relative_day_of_month("hello").is_err());
        assert!(parse_relative_day_of_month("next smarch").is_err());
    }
}
