use chrono::NaiveDate;
use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::tag_no_case,
    character::complete::{char, digit1, multispace1},
    combinator::{map_res, opt},
};

pub(crate) fn parse_absolute(input: &str) -> IResult<&str, NaiveDate> {
    parse_absolute_date_iso
        .or(parse_absolute_date_freedom_units)
        .or(parse_absolute_date_day_month_year)
        .or(parse_absolute_date_month_day_year)
        .parse(input)
}

fn parse_absolute_date_iso(input: &str) -> IResult<&str, NaiveDate> {
    (
        map_res(digit1, str::parse::<u32>),
        char('-'),
        map_res(digit1, str::parse::<u32>),
        char('-'),
        map_res(digit1, str::parse::<u32>),
    )
        .map(|(year, _, month, _, day)| (year, month, day))
        .map_res(|(year, month, day)| {
            NaiveDate::from_ymd_opt(year as i32, month, day).ok_or("invalid date")
        })
        .parse(input)
}

fn parse_absolute_date_freedom_units(input: &str) -> IResult<&str, NaiveDate> {
    (
        map_res(digit1, str::parse::<u32>),
        char('/'),
        map_res(digit1, str::parse::<u32>),
        char('/'),
        map_res(digit1, str::parse::<u32>),
    )
        .map(|(month, _, day, _, year)| (year, month, day))
        .map_res(|(year, month, day)| {
            NaiveDate::from_ymd_opt(year as i32, month, day).ok_or("invalid date")
        })
        .parse(input)
}

fn parse_absolute_date_day_month_year(input: &str) -> IResult<&str, NaiveDate> {
    (
        parse_ordinal_day,
        multispace1,
        parse_month_name,
        multispace1,
        parse_year,
    )
        .map(|(day, _, month, _, year)| (year, month, day))
        .map_res(|(year, month, day)| {
            NaiveDate::from_ymd_opt(year, month, day).ok_or("invalid date")
        })
        .parse(input)
}

fn parse_absolute_date_month_day_year(input: &str) -> IResult<&str, NaiveDate> {
    (
        parse_month_name,
        multispace1,
        parse_ordinal_day,
        multispace1,
        parse_year,
    )
        .map(|(month, _, day, _, year)| (year, month, day))
        .map_res(|(year, month, day)| {
            NaiveDate::from_ymd_opt(year, month, day).ok_or("invalid date")
        })
        .parse(input)
}

fn parse_ordinal_day(input: &str) -> IResult<&str, u32> {
    (
        map_res(digit1, str::parse::<u32>),
        opt(alt((
            tag_no_case("st"),
            tag_no_case("nd"),
            tag_no_case("rd"),
            tag_no_case("th"),
        ))),
    )
        .map(|(day, _)| day)
        .parse(input)
}

fn parse_month_name(input: &str) -> IResult<&str, u32> {
    alt((
        tag_no_case("january").map(|_| 1),
        tag_no_case("february").map(|_| 2),
        tag_no_case("march").map(|_| 3),
        tag_no_case("april").map(|_| 4),
        tag_no_case("may").map(|_| 5),
        tag_no_case("june").map(|_| 6),
        tag_no_case("july").map(|_| 7),
        tag_no_case("august").map(|_| 8),
        tag_no_case("september").map(|_| 9),
        tag_no_case("october").map(|_| 10),
        tag_no_case("november").map(|_| 11),
        tag_no_case("december").map(|_| 12),
    ))
    .parse(input)
}

fn parse_year(input: &str) -> IResult<&str, i32> {
    map_res(digit1, |year: &str| -> Result<i32, &'static str> {
        match year.len() {
            2 => year
                .parse::<i32>()
                .map(|x| 2000 + x)
                .map_err(|_| "invalid year"),
            4 => year.parse::<i32>().map_err(|_| "invalid year"),
            _ => Err("invalid year"),
        }
    })
    .parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_absolute_dates() {
        assert_eq!(
            parse_absolute("2000-10-10"),
            Ok(("", NaiveDate::from_ymd_opt(2000, 10, 10).unwrap()))
        );
        assert_eq!(
            parse_absolute("2000-1-1"),
            Ok(("", NaiveDate::from_ymd_opt(2000, 1, 1).unwrap()))
        );
        assert_eq!(
            parse_absolute("10/20/2000"),
            Ok(("", NaiveDate::from_ymd_opt(2000, 10, 20).unwrap()))
        );
        assert_eq!(
            parse_absolute("1/2/2000"),
            Ok(("", NaiveDate::from_ymd_opt(2000, 1, 2).unwrap()))
        );
        assert_eq!(
            parse_absolute("15th April 2000"),
            Ok(("", NaiveDate::from_ymd_opt(2000, 4, 15).unwrap()))
        );
        assert_eq!(
            parse_absolute("15 April 2000"),
            Ok(("", NaiveDate::from_ymd_opt(2000, 4, 15).unwrap()))
        );
        assert_eq!(
            parse_absolute("April 15th 00"),
            Ok(("", NaiveDate::from_ymd_opt(2000, 4, 15).unwrap()))
        );
        assert_eq!(
            parse_absolute("April 15 00"),
            Ok(("", NaiveDate::from_ymd_opt(2000, 4, 15).unwrap()))
        );
    }

    #[test]
    fn rejects_invalid_absolute_dates() {
        assert!(parse_absolute("2000-13-1").is_err());
        assert!(parse_absolute("2000-2-30").is_err());
        assert!(parse_absolute("13/20/2000").is_err());
        assert!(parse_absolute("2/30/2000").is_err());
        assert!(parse_absolute("31st April 2000").is_err());
        assert!(parse_absolute("April 31st 00").is_err());
        assert!(parse_absolute("Smarch 15th 00").is_err());
        assert!(parse_absolute("2000/10/20").is_err());
        assert!(parse_absolute("hello").is_err());
    }

    #[test]
    fn leaves_remaining_input_for_larger_parsers() {
        assert_eq!(
            parse_absolute("2000-10-10 and more"),
            Ok((" and more", NaiveDate::from_ymd_opt(2000, 10, 10).unwrap()))
        );
        assert_eq!(
            parse_absolute("April 15th 00 please"),
            Ok((" please", NaiveDate::from_ymd_opt(2000, 4, 15).unwrap()))
        );
    }
}
