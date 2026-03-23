use chrono::Month;
use nom::{IResult, Parser, character::complete::multispace1, combinator::opt};

use super::common::{self, RelativeDirection};

pub(super) fn parse_relative_partial_date(input: &str) -> IResult<&str, RelativePartialDate> {
    opt(common::parse_relative_direction
        .and(multispace1)
        .map(|(direction, _)| direction))
    .and(parse_partial_date_body)
    .map(|(direction, (day, month))| RelativePartialDate {
        direction,
        day,
        month,
    })
    .parse(input)
}

pub(super) fn parse_partial_date_body(input: &str) -> IResult<&str, (u32, Month)> {
    (common::parse_ordinal_day, multispace1, common::parse_month)
        .map(|(day, _, month)| (day, month))
        .or(
            (common::parse_month, multispace1, common::parse_ordinal_day)
                .map(|(month, _, day)| (day, month)),
        )
        .parse(input)
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct RelativePartialDate {
    pub direction: Option<RelativeDirection>,
    pub day: u32,
    pub month: Month,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_partial_date_body() {
        assert_eq!(
            parse_partial_date_body("14th april"),
            Ok(("", (14, Month::April)))
        );
        assert_eq!(
            parse_partial_date_body("april 14th"),
            Ok(("", (14, Month::April)))
        );
        assert!(parse_partial_date_body("14th").is_err());
        assert!(parse_partial_date_body("next 14th april").is_err());
    }

    #[test]
    fn parses_relative_partial_dates() {
        assert_eq!(
            parse_relative_partial_date("14th april"),
            Ok((
                "",
                RelativePartialDate {
                    direction: None,
                    day: 14,
                    month: Month::April,
                },
            ))
        );
        assert_eq!(
            parse_relative_partial_date("next 1st December"),
            Ok((
                "",
                RelativePartialDate {
                    direction: Some(RelativeDirection::Future),
                    day: 1,
                    month: Month::December,
                },
            ))
        );
        assert_eq!(
            parse_relative_partial_date("next April 14th"),
            Ok((
                "",
                RelativePartialDate {
                    direction: Some(RelativeDirection::Future),
                    day: 14,
                    month: Month::April,
                },
            ))
        );
        assert_eq!(
            parse_relative_partial_date("last 14 April"),
            Ok((
                "",
                RelativePartialDate {
                    direction: Some(RelativeDirection::Past),
                    day: 14,
                    month: Month::April,
                },
            ))
        );
        assert_eq!(
            parse_relative_partial_date("last April 14"),
            Ok((
                "",
                RelativePartialDate {
                    direction: Some(RelativeDirection::Past),
                    day: 14,
                    month: Month::April,
                },
            ))
        );
    }

    #[test]
    fn parses_partial_dates_case_insensitively() {
        assert_eq!(
            parse_relative_partial_date("Next 14th APRIL"),
            Ok((
                "",
                RelativePartialDate {
                    direction: Some(RelativeDirection::Future),
                    day: 14,
                    month: Month::April,
                },
            ))
        );
    }

    #[test]
    fn leaves_remaining_input_for_larger_parsers() {
        assert_eq!(
            parse_relative_partial_date("14th april please"),
            Ok((
                " please",
                RelativePartialDate {
                    direction: None,
                    day: 14,
                    month: Month::April,
                },
            ))
        );
    }

    #[test]
    fn rejects_bare_days_of_month() {
        assert!(parse_relative_partial_date("14th").is_err());
        assert!(parse_relative_partial_date("next 14").is_err());
        assert!(parse_relative_partial_date("hello").is_err());
    }
}
