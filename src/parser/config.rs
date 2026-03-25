use chrono::{Local, NaiveDate};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum FirstDay {
    #[default]
    Monday,
    Sunday,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ParserConfig {
    pub(crate) first_day: FirstDay,
    pub(crate) next_weekday_means_week: bool,
    pub(crate) next_day_of_month_means_month: bool,
    pub(crate) next_partial_date_means_year: bool,
    pub(crate) last_weekday_means_week: bool,
    pub(crate) last_day_of_month_means_month: bool,
    pub(crate) last_partial_date_means_year: bool,
    pub(crate) today: NaiveDate,
}

impl Default for ParserConfig {
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

impl ParserConfig {
    pub fn new() -> Self {
        Self::default()
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
