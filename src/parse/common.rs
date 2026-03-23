use chrono::{Month, Weekday};
use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::tag_no_case,
    character::complete::digit1,
    combinator::{map_res, opt},
};

#[derive(Debug, Default, PartialEq, Eq)]
pub(crate) enum RelativeDirection {
    #[default]
    Future,
    Past,
}

pub(super) fn parse_relative_direction(input: &str) -> IResult<&str, RelativeDirection> {
    alt((
        tag_no_case("next").map(|_| RelativeDirection::Future),
        tag_no_case("last").map(|_| RelativeDirection::Past),
    ))
    .parse(input)
}

pub(super) fn parse_ordinal_day(input: &str) -> IResult<&str, u32> {
    (
        map_res(digit1, |x: &str| x.parse::<u32>()),
        opt(parse_ordinal_suffix),
    )
        .map(|(day, _)| day)
        .map_res(|day| {
            if (1..=31).contains(&day) {
                Ok(day)
            } else {
                Err("invalid day of month")
            }
        })
        .parse(input)
}

fn parse_ordinal_suffix(input: &str) -> IResult<&str, &str> {
    alt((
        tag_no_case("st"),
        tag_no_case("nd"),
        tag_no_case("rd"),
        tag_no_case("th"),
    ))
    .parse(input)
}

pub(super) fn parse_weekday(input: &str) -> IResult<&str, Weekday> {
    alt((
        alt((tag_no_case("monday"), tag_no_case("mon"))).map(|_| Weekday::Mon),
        alt((tag_no_case("tuesday"), tag_no_case("tue"))).map(|_| Weekday::Tue),
        alt((tag_no_case("wednesday"), tag_no_case("wed"))).map(|_| Weekday::Wed),
        alt((tag_no_case("thursday"), tag_no_case("thu"))).map(|_| Weekday::Thu),
        alt((tag_no_case("friday"), tag_no_case("fri"))).map(|_| Weekday::Fri),
        alt((tag_no_case("saturday"), tag_no_case("sat"))).map(|_| Weekday::Sat),
        alt((tag_no_case("sunday"), tag_no_case("sun"))).map(|_| Weekday::Sun),
    ))
    .parse(input)
}

pub(super) fn parse_month(input: &str) -> IResult<&str, Month> {
    alt((
        tag_no_case("january").map(|_| Month::January),
        tag_no_case("february").map(|_| Month::February),
        tag_no_case("march").map(|_| Month::March),
        tag_no_case("april").map(|_| Month::April),
        tag_no_case("may").map(|_| Month::May),
        tag_no_case("june").map(|_| Month::June),
        tag_no_case("july").map(|_| Month::July),
        tag_no_case("august").map(|_| Month::August),
        tag_no_case("september").map(|_| Month::September),
        tag_no_case("october").map(|_| Month::October),
        tag_no_case("november").map(|_| Month::November),
        tag_no_case("december").map(|_| Month::December),
    ))
    .parse(input)
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
    pub(super) fn parse(input: &str) -> IResult<&str, Self> {
        alt((
            tag_no_case("day").map(|_| Self::Day),
            tag_no_case("week").map(|_| Self::Week),
            tag_no_case("month").map(|_| Self::Month),
            tag_no_case("year").map(|_| Self::Year),
        ))
        .parse(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_relative_directions() {
        assert_eq!(
            parse_relative_direction("next"),
            Ok(("", RelativeDirection::Future))
        );
        assert_eq!(
            parse_relative_direction("last"),
            Ok(("", RelativeDirection::Past))
        );
    }

    #[test]
    fn parses_ordinal_days() {
        assert_eq!(parse_ordinal_day("1"), Ok(("", 1)));
        assert_eq!(parse_ordinal_day("14th"), Ok(("", 14)));
        assert_eq!(parse_ordinal_day("22nd"), Ok(("", 22)));
    }

    #[test]
    fn rejects_invalid_ordinal_days() {
        assert!(parse_ordinal_day("0").is_err());
        assert!(parse_ordinal_day("32nd").is_err());
        assert!(parse_ordinal_day("hello").is_err());
    }

    #[test]
    fn parses_months() {
        assert_eq!(parse_month("january"), Ok(("", Month::January)));
        assert_eq!(parse_month("april"), Ok(("", Month::April)));
        assert_eq!(parse_month("December"), Ok(("", Month::December)));
    }

    #[test]
    fn rejects_invalid_months() {
        assert!(parse_month("month").is_err());
        assert!(parse_month("jan").is_err());
        assert!(parse_month("hello").is_err());
    }

    #[test]
    fn parses_interval_units() {
        assert_eq!(IntervalUnit::parse("day"), Ok(("", IntervalUnit::Day)));
        assert_eq!(IntervalUnit::parse("week"), Ok(("", IntervalUnit::Week)));
        assert_eq!(IntervalUnit::parse("month"), Ok(("", IntervalUnit::Month)));
        assert_eq!(IntervalUnit::parse("year"), Ok(("", IntervalUnit::Year)));
    }

    #[test]
    fn rejects_invalid_interval_units() {
        assert_eq!(IntervalUnit::parse("days"), Ok(("s", IntervalUnit::Day)));
        assert!(IntervalUnit::parse("hello").is_err());
    }

    #[test]
    fn parses_full_weekday_names() {
        assert_eq!(parse_weekday("monday"), Ok(("", Weekday::Mon)));
        assert_eq!(parse_weekday("tuesday"), Ok(("", Weekday::Tue)));
        assert_eq!(parse_weekday("wednesday"), Ok(("", Weekday::Wed)));
        assert_eq!(parse_weekday("thursday"), Ok(("", Weekday::Thu)));
        assert_eq!(parse_weekday("friday"), Ok(("", Weekday::Fri)));
        assert_eq!(parse_weekday("saturday"), Ok(("", Weekday::Sat)));
        assert_eq!(parse_weekday("sunday"), Ok(("", Weekday::Sun)));
    }

    #[test]
    fn parses_common_weekday_abbreviations() {
        assert_eq!(parse_weekday("mon"), Ok(("", Weekday::Mon)));
        assert_eq!(parse_weekday("tue"), Ok(("", Weekday::Tue)));
        assert_eq!(parse_weekday("wed"), Ok(("", Weekday::Wed)));
        assert_eq!(parse_weekday("thu"), Ok(("", Weekday::Thu)));
        assert_eq!(parse_weekday("fri"), Ok(("", Weekday::Fri)));
        assert_eq!(parse_weekday("sat"), Ok(("", Weekday::Sat)));
        assert_eq!(parse_weekday("sun"), Ok(("", Weekday::Sun)));
    }

    #[test]
    fn parses_weekdays_case_insensitively() {
        assert_eq!(parse_weekday("Monday"), Ok(("", Weekday::Mon)));
        assert_eq!(parse_weekday("THU"), Ok(("", Weekday::Thu)));
        assert_eq!(parse_weekday("SunDay"), Ok(("", Weekday::Sun)));
    }

    #[test]
    fn leaves_remaining_input_for_larger_parsers() {
        assert_eq!(
            parse_weekday("monday next week"),
            Ok((" next week", Weekday::Mon))
        );
    }

    #[test]
    fn rejects_invalid_weekdays() {
        assert!(parse_weekday("hello").is_err());
        assert!(parse_weekday("mo").is_err());
    }
}
