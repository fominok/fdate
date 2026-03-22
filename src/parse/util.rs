use chrono::Weekday;
use nom::{IResult, Parser, bytes::complete::tag_no_case};

pub(crate) fn parse_weekday(input: &str) -> IResult<&str, Weekday> {
    (tag_no_case("monday")
        .or(tag_no_case("mon"))
        .map(|_| Weekday::Mon))
    .or(tag_no_case("tuesday")
        .or(tag_no_case("tue"))
        .map(|_| Weekday::Tue))
    .or(tag_no_case("wednesday")
        .or(tag_no_case("wed"))
        .map(|_| Weekday::Wed))
    .or(tag_no_case("thursday")
        .or(tag_no_case("thu"))
        .map(|_| Weekday::Thu))
    .or(tag_no_case("friday")
        .or(tag_no_case("fri"))
        .map(|_| Weekday::Fri))
    .or(tag_no_case("saturday")
        .or(tag_no_case("sat"))
        .map(|_| Weekday::Sat))
    .or(tag_no_case("sunday")
        .or(tag_no_case("sun"))
        .map(|_| Weekday::Sun))
    .parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;

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
