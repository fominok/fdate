use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::tag_no_case,
    character::complete::{char, digit1, multispace1},
    combinator::{map_res, opt},
};

pub(crate) fn parse_relative_interval(input: &str) -> IResult<&str, RelativeInterval> {
    parse_relative_interval_literals
        .or(parse_relative_interval_past)
        .or(parse_relative_interval_future)
        .parse(input)
}

#[derive(Debug, Default, PartialEq, Eq)]
pub(crate) enum IntervalDirection {
    #[default]
    Future,
    Past,
}

#[derive(Debug, Default, PartialEq, Eq)]
pub(crate) enum IntervalUnit {
    #[default]
    Day,
    Week,
    Month,
    Year,
}

impl IntervalUnit {
    fn parse(input: &str) -> IResult<&str, Self> {
        alt((
            tag_no_case("day").map(|_| Self::Day),
            tag_no_case("week").map(|_| Self::Week),
            tag_no_case("month").map(|_| Self::Month),
            tag_no_case("year").map(|_| Self::Year),
        ))
        .parse(input)
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
pub(crate) struct RelativeInterval {
    pub direction: IntervalDirection,
    pub value: u32,
    pub unit: IntervalUnit,
}

fn parse_relative_interval_literals(input: &str) -> IResult<&str, RelativeInterval> {
    tag_no_case("today")
        .map(|_| RelativeInterval::default())
        .or(tag_no_case("tomorrow").map(|_| RelativeInterval {
            value: 1,
            ..Default::default()
        }))
        .or(tag_no_case("yesterday").map(|_| RelativeInterval {
            value: 1,
            direction: IntervalDirection::Past,
            ..Default::default()
        }))
        .parse(input)
}

fn parse_relative_interval_future(input: &str) -> IResult<&str, RelativeInterval> {
    let (input, _) = opt(tag_no_case("in").and(multispace1)).parse(input)?;
    let (input, value_opt) = opt(parse_relative_interval_value.and(multispace1)).parse(input)?;
    let value = value_opt.map(|(x, _)| x).unwrap_or_default();
    let (input, unit) = IntervalUnit::parse(input)?;
    let (input, _) = opt(char('s')).parse(input)?;

    Ok((
        input,
        RelativeInterval {
            direction: IntervalDirection::Future,
            value,
            unit,
        },
    ))
}

fn parse_relative_interval_past(input: &str) -> IResult<&str, RelativeInterval> {
    let (input, (value, _)) = parse_relative_interval_value
        .and(multispace1)
        .parse(input)?;
    let (input, unit) = IntervalUnit::parse(input)?;
    let (input, _) = opt(char('s')).parse(input)?;
    let (input, _) = multispace1.and(tag_no_case("ago")).parse(input)?;

    Ok((
        input,
        RelativeInterval {
            direction: IntervalDirection::Past,
            value,
            unit,
        },
    ))
}

fn parse_relative_interval_value(input: &str) -> IResult<&str, u32> {
    tag_no_case("a")
        .map(|_| 1)
        .or(map_res(digit1, str::parse::<u32>))
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
                    direction: IntervalDirection::Future,
                    value: 0,
                    unit: IntervalUnit::Day,
                },
            ))
        );
        assert_eq!(
            parse_relative_interval("tomorrow"),
            Ok((
                "",
                RelativeInterval {
                    direction: IntervalDirection::Future,
                    value: 1,
                    unit: IntervalUnit::Day,
                },
            ))
        );
        assert_eq!(
            parse_relative_interval("yesterday"),
            Ok((
                "",
                RelativeInterval {
                    direction: IntervalDirection::Past,
                    value: 1,
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
                    direction: IntervalDirection::Future,
                    value: 3,
                    unit: IntervalUnit::Day,
                },
            ))
        );
        assert_eq!(
            parse_relative_interval("in a week"),
            Ok((
                "",
                RelativeInterval {
                    direction: IntervalDirection::Future,
                    value: 1,
                    unit: IntervalUnit::Week,
                },
            ))
        );
        assert_eq!(
            parse_relative_interval("month"),
            Ok((
                "",
                RelativeInterval {
                    direction: IntervalDirection::Future,
                    value: 0,
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
                    direction: IntervalDirection::Past,
                    value: 2,
                    unit: IntervalUnit::Year,
                },
            ))
        );
        assert_eq!(
            parse_relative_interval("a month ago"),
            Ok((
                "",
                RelativeInterval {
                    direction: IntervalDirection::Past,
                    value: 1,
                    unit: IntervalUnit::Month,
                },
            ))
        );
        assert_eq!(
            parse_relative_interval("a week ago"),
            Ok((
                "",
                RelativeInterval {
                    direction: IntervalDirection::Past,
                    value: 1,
                    unit: IntervalUnit::Week,
                },
            ))
        );
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
                " ago",
                RelativeInterval {
                    direction: IntervalDirection::Future,
                    value: 0,
                    unit: IntervalUnit::Day,
                },
            ))
        );
        assert_eq!(
            parse_relative_interval("2 years ago exactly"),
            Ok((
                " exactly",
                RelativeInterval {
                    direction: IntervalDirection::Past,
                    value: 2,
                    unit: IntervalUnit::Year,
                },
            ))
        );
    }
}
