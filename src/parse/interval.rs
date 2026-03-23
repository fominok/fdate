use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::tag_no_case,
    character::complete::{char, digit1, multispace1},
    combinator::{map_res, opt},
};

use super::common::{IntervalUnit, RelativeDirection};

pub(super) fn parse_relative_interval(input: &str) -> IResult<&str, RelativeInterval> {
    alt((
        parse_relative_interval_literals,
        parse_non_literal_relative_interval,
    ))
    .parse(input)
}

pub(super) fn parse_non_literal_relative_interval(input: &str) -> IResult<&str, RelativeInterval> {
    parse_relative_interval_past
        .or(RelativeIntervalFutureParser::new(false).parser())
        .parse(input)
}

/// A slight variation of [parse_non_literal_relative_interval] to forbid usage
/// of
// the shortest expressions like "month" that doesn't fit in larger contexts
// which expect something no less than "in (a) month".
pub(super) fn parse_explicit_non_literal_relative_interval(
    input: &str,
) -> IResult<&str, RelativeInterval> {
    parse_relative_interval_past
        .or(RelativeIntervalFutureParser::new(true).parser())
        .parse(input)
}

#[derive(Debug, Default, PartialEq, Eq)]
pub(crate) struct RelativeInterval {
    pub direction: RelativeDirection,
    pub distance: u32,
    pub unit: IntervalUnit,
}

fn parse_relative_interval_literals(input: &str) -> IResult<&str, RelativeInterval> {
    alt((
        tag_no_case("today").map(|_| RelativeInterval::default()),
        tag_no_case("tomorrow").map(|_| RelativeInterval {
            distance: 1,
            ..Default::default()
        }),
        tag_no_case("yesterday").map(|_| RelativeInterval {
            distance: 1,
            direction: RelativeDirection::Past,
            ..Default::default()
        }),
    ))
    .parse(input)
}

struct RelativeIntervalFutureParser {
    explicit: bool,
}

impl RelativeIntervalFutureParser {
    fn new(explicit: bool) -> Self {
        Self { explicit }
    }

    fn parser<'s>(&self) -> impl Fn(&'s str) -> IResult<&'s str, RelativeInterval> {
        |input| {
            let (input, prefix_opt) = opt(tag_no_case("in").and(multispace1)).parse(input)?;
            let (input, value_opt) =
                opt(parse_relative_interval_value.and(multispace1)).parse(input)?;

            if self.explicit && (prefix_opt.is_none() || value_opt.is_none()) {
                return Err(nom::Err::Error(nom::error::Error::new(
                    input,
                    nom::error::ErrorKind::Tag,
                )));
            }
            let value = value_opt.map(|(x, _)| x).unwrap_or(1);
            let (input, unit) = IntervalUnit::parse(input)?;
            let (input, _) = opt(char('s')).parse(input)?;

            Ok((
                input,
                RelativeInterval {
                    direction: RelativeDirection::Future,
                    distance: value,
                    unit,
                },
            ))
        }
    }
}

fn parse_relative_interval_past(input: &str) -> IResult<&str, RelativeInterval> {
    let (input, value) = opt(parse_relative_interval_value.and(multispace1))
        .map(|value_opt| value_opt.map(|(value, _)| value).unwrap_or(1))
        .parse(input)?;
    let (input, unit) = IntervalUnit::parse(input)?;
    let (input, _) = opt(char('s')).parse(input)?;
    let (input, _) = multispace1.and(tag_no_case("ago")).parse(input)?;

    Ok((
        input,
        RelativeInterval {
            direction: RelativeDirection::Past,
            distance: value,
            unit,
        },
    ))
}

fn parse_relative_interval_value(input: &str) -> IResult<&str, u32> {
    alt((
        tag_no_case("a").map(|_| 1),
        map_res(digit1, str::parse::<u32>),
    ))
    .parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_relative_interval_literals() {
        assert_eq!(
            parse_relative_interval("today"),
            Ok((
                "",
                RelativeInterval {
                    direction: RelativeDirection::Future,
                    distance: 0,
                    unit: IntervalUnit::Day,
                },
            ))
        );
        assert_eq!(
            parse_relative_interval("tomorrow"),
            Ok((
                "",
                RelativeInterval {
                    direction: RelativeDirection::Future,
                    distance: 1,
                    unit: IntervalUnit::Day,
                },
            ))
        );
        assert_eq!(
            parse_relative_interval("yesterday"),
            Ok((
                "",
                RelativeInterval {
                    direction: RelativeDirection::Past,
                    distance: 1,
                    unit: IntervalUnit::Day,
                },
            ))
        );
    }

    #[test]
    fn parses_future_relative_intervals() {
        assert_eq!(
            parse_relative_interval("in 3 days"),
            Ok((
                "",
                RelativeInterval {
                    direction: RelativeDirection::Future,
                    distance: 3,
                    unit: IntervalUnit::Day,
                },
            ))
        );
        assert_eq!(
            parse_relative_interval("in a week"),
            Ok((
                "",
                RelativeInterval {
                    direction: RelativeDirection::Future,
                    distance: 1,
                    unit: IntervalUnit::Week,
                },
            ))
        );
        assert_eq!(
            parse_relative_interval("month"),
            Ok((
                "",
                RelativeInterval {
                    direction: RelativeDirection::Future,
                    distance: 1,
                    unit: IntervalUnit::Month,
                },
            ))
        );
    }

    #[test]
    fn parses_past_relative_intervals() {
        assert_eq!(
            parse_relative_interval("2 years ago"),
            Ok((
                "",
                RelativeInterval {
                    direction: RelativeDirection::Past,
                    distance: 2,
                    unit: IntervalUnit::Year,
                },
            ))
        );
        assert_eq!(
            parse_relative_interval("a month ago"),
            Ok((
                "",
                RelativeInterval {
                    direction: RelativeDirection::Past,
                    distance: 1,
                    unit: IntervalUnit::Month,
                },
            ))
        );
        assert_eq!(
            parse_relative_interval("a week ago"),
            Ok((
                "",
                RelativeInterval {
                    direction: RelativeDirection::Past,
                    distance: 1,
                    unit: IntervalUnit::Week,
                },
            ))
        );
    }

    #[test]
    fn parses_non_literal_relative_intervals() {
        assert_eq!(
            parse_non_literal_relative_interval("in a week"),
            Ok((
                "",
                RelativeInterval {
                    direction: RelativeDirection::Future,
                    distance: 1,
                    unit: IntervalUnit::Week,
                },
            ))
        );
        assert_eq!(
            parse_non_literal_relative_interval("2 years ago"),
            Ok((
                "",
                RelativeInterval {
                    direction: RelativeDirection::Past,
                    distance: 2,
                    unit: IntervalUnit::Year,
                },
            ))
        );
        assert!(parse_non_literal_relative_interval("today").is_err());
    }

    #[test]
    fn rejects_invalid_relative_intervals() {
        assert!(parse_relative_interval("in several days").is_err());
        assert!(parse_relative_interval("a 15 days ago").is_err());
        assert!(parse_relative_interval("hello").is_err());
    }

    #[test]
    fn leaves_remaining_input_for_larger_parsers() {
        assert_eq!(
            parse_relative_interval("days ago"),
            Ok((
                "",
                RelativeInterval {
                    direction: RelativeDirection::Past,
                    distance: 1,
                    unit: IntervalUnit::Day,
                },
            ))
        );
        assert_eq!(
            parse_relative_interval("2 years ago exactly"),
            Ok((
                " exactly",
                RelativeInterval {
                    direction: RelativeDirection::Past,
                    distance: 2,
                    unit: IntervalUnit::Year,
                },
            ))
        );
    }
}
