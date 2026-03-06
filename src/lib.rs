//! _Fdate_ is a library for searching relative dates in English. It attempts to
//! be simple and unambigious yet provides a bit of flexibility with
//! configuration.

use chrono::{Local, NaiveDate};

/// Returns a date on successful parsing with defaults applied:
/// 1. Week starts with Monday,
/// 2. Calls for [chrono::Local::now],
/// 3. `next` and `last` mean the closest but today, e. g. `next Wednesday` will
///    mean tomorrow if today is Tuesday.
pub fn parse(input: &str) -> Option<NaiveDate> {
    Parser::new().parse(input)
}

/// Configurable parser in case defaults don't suit.
pub struct Parser {
    first_day: FirstDay,
    next_weekday_means_week: bool,
    next_day_of_month_means_month: bool,
    next_partial_date_means_year: bool,
    last_weekday_means_week: bool,
    last_day_of_month_means_month: bool,
    last_partial_date_means_year: bool,
    today: NaiveDate,
}

impl Default for Parser {
    fn default() -> Self {
        Self {
            first_day: FirstDay::Monday,
            next_weekday_means_week: false,
            next_day_of_month_means_month: false,
            next_partial_date_means_year: false,
            last_weekday_means_week: false,
            last_day_of_month_means_month: false,
            last_partial_date_means_year: false,
            today: Local::now().date_naive(),
        }
    }
}

impl Parser {
    /// Create [Parser] with default settings that will behave as [parse] unless
    /// changed.
    pub fn new() -> Self {
        Self::default()
    }

    /// Attempts to parse input into a date.
    pub fn parse(&self, input: &str) -> Option<NaiveDate> {
        todo!()
    }

    /// Set today's date explicitly
    pub fn with_today(&mut self, today: NaiveDate) -> &mut Self {
        self.today = today;
        self
    }

    /// First day of week is Monday
    pub fn week_starts_monday(&mut self) -> &mut Self {
        self.first_day = FirstDay::Monday;
        self
    }

    /// First day of week is Sunday
    pub fn week_starts_sunday(&mut self) -> &mut Self {
        self.first_day = FirstDay::Sunday;
        self
    }

    /// `next Wednesday` will be in 8 days if today is Tuesday
    pub fn next_weekday_means_week(&mut self) -> &mut Self {
        self.next_weekday_means_week = true;
        self
    }

    /// `next Wednesday` will be tomorrow if today is Tuesday (default)
    pub fn next_weekday_means_closest(&mut self) -> &mut Self {
        self.next_weekday_means_week = false;
        self
    }

    /// `next 14th` will be next month if today is 13th (in ~31 days)
    pub fn next_day_of_month_means_month(&mut self) -> &mut Self {
        self.next_day_of_month_means_month = true;
        self
    }

    /// `next 14th` will be tomorrow if today is 13th (default)
    pub fn next_day_of_month_means_closest(&mut self) -> &mut Self {
        self.next_day_of_month_means_month = false;
        self
    }

    /// `next April 14th` will be in 2027 if it's 2026 now
    pub fn next_partial_date_means_year(&mut self) -> &mut Self {
        self.next_partial_date_means_year = true;
        self
    }

    /// `next April 14th` will be in one month if it's March now (default).
    pub fn next_partial_date_means_closest(&mut self) -> &mut Self {
        self.next_partial_date_means_year = false;
        self
    }

    /// `last Wednesday` was 8 days ago if today is Thursday
    pub fn last_weekday_means_week(&mut self) -> &mut Self {
        self.last_weekday_means_week = true;
        self
    }

    /// `last Wednesday` was yesterday if today is Thursday (default)
    pub fn last_weekday_means_closest(&mut self) -> &mut Self {
        self.last_weekday_means_week = false;
        self
    }

    /// `last 14th` was in the previous month if today is 15th (in ~31 days)
    pub fn last_day_of_month_means_month(&mut self) -> &mut Self {
        self.last_day_of_month_means_month = true;
        self
    }

    /// `last 14th` was yesterday if today is 15th (default)
    pub fn last_day_of_month_means_closest(&mut self) -> &mut Self {
        self.last_day_of_month_means_month = false;
        self
    }

    /// `last April 14th` was in 2025 if it's 2026 now
    pub fn last_partial_date_means_year(&mut self) -> &mut Self {
        self.last_partial_date_means_year = true;
        self
    }

    /// `next April 14th` was in the previous month if it's May now (default).
    pub fn last_partial_date_means_closest(&mut self) -> &mut Self {
        self.last_partial_date_means_year = false;
        self
    }
}

enum FirstDay {
    Monday,
    Sunday,
}
