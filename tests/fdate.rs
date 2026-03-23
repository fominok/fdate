use chrono::NaiveDate;
use fdate::Parser;

fn base_date() -> NaiveDate {
    NaiveDate::from_ymd_opt(2025, 5, 5).unwrap()
}

fn parser_with_base_date() -> Parser {
    let mut parser = Parser::new();
    parser.with_today(base_date());
    parser
}

#[test]
fn parses_absolute_dates() {
    let parser = parser_with_base_date();

    assert_eq!(
        parser.parse("2025-05-20"),
        Some(NaiveDate::from_ymd_opt(2025, 5, 20).unwrap())
    );
    assert_eq!(
        parser.parse("5/20/2025"),
        Some(NaiveDate::from_ymd_opt(2025, 5, 20).unwrap())
    );
    assert_eq!(
        parser.parse("20th May 2025"),
        Some(NaiveDate::from_ymd_opt(2025, 5, 20).unwrap())
    );
}

#[test]
fn parses_relative_intervals() {
    let parser = parser_with_base_date();

    assert_eq!(
        parser.parse("in 2 weeks"),
        Some(NaiveDate::from_ymd_opt(2025, 5, 19).unwrap())
    );
    assert_eq!(
        parser.parse("tomorrow"),
        Some(NaiveDate::from_ymd_opt(2025, 5, 6).unwrap())
    );
}

#[test]
fn resolves_past_relative_intervals_into_the_past() {
    let parser = parser_with_base_date();

    assert_eq!(
        parser.parse("yesterday"),
        Some(NaiveDate::from_ymd_opt(2025, 5, 4).unwrap())
    );
    assert_eq!(
        parser.parse("a week ago"),
        Some(NaiveDate::from_ymd_opt(2025, 4, 28).unwrap())
    );
    assert_eq!(
        parser.parse("2 months ago"),
        Some(NaiveDate::from_ymd_opt(2025, 3, 5).unwrap())
    );
}

#[test]
fn parses_relative_weekdays_with_default_closest_semantics() {
    let parser = parser_with_base_date();

    assert_eq!(
        parser.parse("wednesday"),
        Some(NaiveDate::from_ymd_opt(2025, 5, 7).unwrap())
    );
    assert_eq!(
        parser.parse("next wednesday"),
        Some(NaiveDate::from_ymd_opt(2025, 5, 7).unwrap())
    );
    assert_eq!(
        parser.parse("last wednesday"),
        Some(NaiveDate::from_ymd_opt(2025, 4, 30).unwrap())
    );
}

#[test]
fn parses_relative_weekdays_with_calendar_week_settings() {
    let mut parser = parser_with_base_date();
    parser.next_weekday_means_week().last_weekday_means_week();

    assert_eq!(
        parser.parse("wednesday"),
        Some(NaiveDate::from_ymd_opt(2025, 5, 7).unwrap())
    );
    assert_eq!(
        parser.parse("next wednesday"),
        Some(NaiveDate::from_ymd_opt(2025, 5, 14).unwrap())
    );
    assert_eq!(
        parser.parse("last wednesday"),
        Some(NaiveDate::from_ymd_opt(2025, 4, 30).unwrap())
    );
}

#[test]
fn parses_relative_weekdays_with_sunday_weeks() {
    let mut parser = parser_with_base_date();
    parser
        .week_starts_sunday()
        .next_weekday_means_week()
        .last_weekday_means_week();

    assert_eq!(
        parser.parse("next sunday"),
        Some(NaiveDate::from_ymd_opt(2025, 5, 11).unwrap())
    );
    assert_eq!(
        parser.parse("last sunday"),
        Some(NaiveDate::from_ymd_opt(2025, 4, 27).unwrap())
    );
}

#[test]
fn parses_bare_days_of_month_with_default_closest_semantics() {
    let parser = parser_with_base_date();

    assert_eq!(
        parser.parse("14th"),
        Some(NaiveDate::from_ymd_opt(2025, 5, 14).unwrap())
    );
    assert_eq!(
        parser.parse("next 14th"),
        Some(NaiveDate::from_ymd_opt(2025, 5, 14).unwrap())
    );
    assert_eq!(
        parser.parse("last 14th"),
        Some(NaiveDate::from_ymd_opt(2025, 4, 14).unwrap())
    );
}

#[test]
fn parses_bare_days_of_month_with_calendar_month_settings() {
    let mut parser = parser_with_base_date();
    parser
        .next_day_of_month_means_month()
        .last_day_of_month_means_month();

    assert_eq!(
        parser.parse("14th"),
        Some(NaiveDate::from_ymd_opt(2025, 5, 14).unwrap())
    );
    assert_eq!(
        parser.parse("next 14th"),
        Some(NaiveDate::from_ymd_opt(2025, 6, 14).unwrap())
    );
    assert_eq!(
        parser.parse("last 14th"),
        Some(NaiveDate::from_ymd_opt(2025, 4, 14).unwrap())
    );
}

#[test]
fn keeps_partial_dates_on_partial_date_semantics_when_day_of_month_settings_change() {
    let mut parser = parser_with_base_date();
    parser
        .next_day_of_month_means_month()
        .last_day_of_month_means_month();

    assert_eq!(
        parser.parse("next june 14th"),
        Some(NaiveDate::from_ymd_opt(2025, 6, 14).unwrap())
    );
    assert_eq!(
        parser.parse("last june 14th"),
        Some(NaiveDate::from_ymd_opt(2024, 6, 14).unwrap())
    );
}

#[test]
fn parses_partial_dates_with_default_closest_semantics() {
    let parser = parser_with_base_date();

    assert_eq!(
        parser.parse("14th june"),
        Some(NaiveDate::from_ymd_opt(2025, 6, 14).unwrap())
    );
    assert_eq!(
        parser.parse("june 14th"),
        Some(NaiveDate::from_ymd_opt(2025, 6, 14).unwrap())
    );
    assert_eq!(
        parser.parse("next june 14th"),
        Some(NaiveDate::from_ymd_opt(2025, 6, 14).unwrap())
    );
    assert_eq!(
        parser.parse("next 14th june"),
        Some(NaiveDate::from_ymd_opt(2025, 6, 14).unwrap())
    );
    assert_eq!(
        parser.parse("last june 14th"),
        Some(NaiveDate::from_ymd_opt(2024, 6, 14).unwrap())
    );
}

#[test]
fn parses_partial_dates_with_calendar_year_settings() {
    let mut parser = parser_with_base_date();
    parser
        .next_partial_date_means_year()
        .last_partial_date_means_year();

    assert_eq!(
        parser.parse("14th june"),
        Some(NaiveDate::from_ymd_opt(2025, 6, 14).unwrap())
    );
    assert_eq!(
        parser.parse("next june 14th"),
        Some(NaiveDate::from_ymd_opt(2026, 6, 14).unwrap())
    );
    assert_eq!(
        parser.parse("next 14th june"),
        Some(NaiveDate::from_ymd_opt(2026, 6, 14).unwrap())
    );
    assert_eq!(
        parser.parse("last june 14th"),
        Some(NaiveDate::from_ymd_opt(2024, 6, 14).unwrap())
    );
}

#[test]
fn keeps_bare_days_of_month_on_day_of_month_semantics_when_partial_date_settings_change() {
    let mut parser = parser_with_base_date();
    parser
        .next_partial_date_means_year()
        .last_partial_date_means_year();

    assert_eq!(
        parser.parse("next 14th"),
        Some(NaiveDate::from_ymd_opt(2025, 5, 14).unwrap())
    );
    assert_eq!(
        parser.parse("last 14th"),
        Some(NaiveDate::from_ymd_opt(2025, 4, 14).unwrap())
    );
}

#[test]
fn parses_relative_interval_dates_for_weeks() {
    let parser = parser_with_base_date();

    assert_eq!(
        parser.parse("wednesday this week"),
        Some(NaiveDate::from_ymd_opt(2025, 5, 7).unwrap())
    );
    assert_eq!(
        parser.parse("friday in 2 weeks"),
        Some(NaiveDate::from_ymd_opt(2025, 5, 23).unwrap())
    );
    assert_eq!(
        parser.parse("tuesday a week ago"),
        Some(NaiveDate::from_ymd_opt(2025, 4, 29).unwrap())
    );
}

#[test]
fn parses_relative_interval_dates_for_months() {
    let parser = parser_with_base_date();

    assert_eq!(
        parser.parse("14th this month"),
        Some(NaiveDate::from_ymd_opt(2025, 5, 14).unwrap())
    );
    assert_eq!(
        parser.parse("17 in 2 months"),
        Some(NaiveDate::from_ymd_opt(2025, 7, 17).unwrap())
    );
    assert_eq!(
        parser.parse("10 a month ago"),
        Some(NaiveDate::from_ymd_opt(2025, 4, 10).unwrap())
    );
}

#[test]
fn parses_relative_interval_dates_for_years() {
    let parser = parser_with_base_date();

    assert_eq!(
        parser.parse("1st may this year"),
        Some(NaiveDate::from_ymd_opt(2025, 5, 1).unwrap())
    );
    assert_eq!(
        parser.parse("10th july in 2 years"),
        Some(NaiveDate::from_ymd_opt(2027, 7, 10).unwrap())
    );
    assert_eq!(
        parser.parse("may 1 a year ago"),
        Some(NaiveDate::from_ymd_opt(2024, 5, 1).unwrap())
    );
}

#[test]
fn returns_none_for_impossible_partial_dates_and_interval_dates() {
    let parser = parser_with_base_date();

    assert_eq!(parser.parse("31st april"), None);
    assert_eq!(parser.parse("31st april this year"), None);
    assert_eq!(parser.parse("february 29 in a year"), None);
}

#[test]
fn rejects_interval_date_queries_missing_relative_interval_phrasing() {
    let parser = parser_with_base_date();

    assert_eq!(parser.parse("monday week"), None);
    assert_eq!(parser.parse("14th month"), None);
    assert_eq!(parser.parse("may 1 year"), None);
}

#[test]
fn clamps_days_when_months_do_not_have_target_day() {
    let mut parser = Parser::new();
    parser.with_today(NaiveDate::from_ymd_opt(2025, 4, 30).unwrap());

    assert_eq!(
        parser.parse("31st"),
        Some(NaiveDate::from_ymd_opt(2025, 5, 31).unwrap())
    );
    assert_eq!(
        parser.parse("31st this month"),
        Some(NaiveDate::from_ymd_opt(2025, 4, 30).unwrap())
    );
    assert_eq!(
        parser.parse("31st in a month"),
        Some(NaiveDate::from_ymd_opt(2025, 5, 31).unwrap())
    );
}
