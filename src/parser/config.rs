use chrono::{Local, NaiveDate};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum FirstDay {
    #[default]
    Monday,
    Sunday,
}

fn default_date() -> NaiveDate {
    Local::now().date_naive()
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ParserConfig {
    pub first_day: FirstDay,
    pub next_weekday_means_week: bool,
    pub next_day_of_month_means_month: bool,
    pub next_partial_date_means_year: bool,
    pub last_weekday_means_week: bool,
    pub last_day_of_month_means_month: bool,
    pub last_partial_date_means_year: bool,
    #[cfg_attr(feature = "serde", serde(skip, default = "default_date"))]
    pub today: NaiveDate,
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
            today: default_date(),
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
