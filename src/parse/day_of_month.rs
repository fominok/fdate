use nom::{
    IResult, Parser,
    character::complete::multispace1,
    combinator::{not, opt, peek},
};

use super::common::{self, RelativeDirection};

pub(super) fn parse_relative_day_of_month(input: &str) -> IResult<&str, RelativeDayOfMonth> {
    opt(common::parse_relative_direction
        .and(multispace1)
        .map(|(direction, _)| direction))
    .and(
        common::parse_ordinal_day
            .and(not(peek(multispace1.and(common::parse_month))))
            .map(|(day, _)| day),
    )
    .map(|(direction, day)| RelativeDayOfMonth { direction, day })
    .parse(input)
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct RelativeDayOfMonth {
    pub direction: Option<RelativeDirection>,
    pub day: u32,
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
                },
            ))
        );
    }

    #[test]
    fn rejects_partial_dates() {
        assert!(parse_relative_day_of_month("14th april").is_err());
        assert!(parse_relative_day_of_month("next 1st December").is_err());
        assert!(parse_relative_day_of_month("next April 14th").is_err());
        assert!(parse_relative_day_of_month("last 14 April").is_err());
        assert!(parse_relative_day_of_month("last April 14").is_err());
    }

    #[test]
    fn leaves_remaining_input_for_larger_parsers() {
        assert_eq!(
            parse_relative_day_of_month("14th please"),
            Ok((
                " please",
                RelativeDayOfMonth {
                    direction: None,
                    day: 14,
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
