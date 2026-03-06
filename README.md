# fdate


Natural language date parsing made human-friendly.

This library addresses the challenge of interpreting relative dates from everyday
language, specifically designed for straightforward use cases commonly found in scheduling
and journaling contexts.

# Examples

## Absolute

While focused on relative dates, the library still supports absolute date parsing. These
do not account for today’s date.

- 15(th) April 2000
- 2000-04-15
- April 15(th) 00

## Relative

Relative queries depend on today's date, yet there are several different logic paths to
follow from there.

### Intervals

This syntax allows for the addition or subtraction of a specific number of days from
today's date. In the case of month intervals, the library attempts to preserve the
calendar day where possible. If the day does not fit (e.g., moving one month away from
the 31st of the current month), it will default to the greatest day that fits into the
resulting month. This ensures it won't overflow into the following month or skip ahead by
two months.

- Today / (in) 0 day(s) / 0 days(s) ago / (in) 0 year(s) / 0 month(s) ago
- Tomorrow / (in) 1 day(s)
- Yesterday / 1 day(s) ago
- (in) 1 month(s)
- 2 month(s) ago

### Relative days of week

#### Default behavior

By default this query finds the closest day of the week in both directions that is not
today. At most, the target day will be exactly 7 days away if it matches today's day
of the week. In general, for `fdate`, "next" means the closest occurrence excluding the
current day; this logic applies to the following sections as well.

- (next) wednesday
- last wednesday

#### Alternative behavior

This behavior can be customized, as there is no universal consensus on whether "next"
means "closest in the future" or "next calendar week". Alternatively, `next` and `last`
can apply on a week-by-week basis.

- next wednesday (equals to `wednesday next week`)
- last wednesday (equals to `wednesday last week`)
- wednesday (closest Wednesday in the future, same as `next wednesday` in default mode)

### Relative days of month

#### Default behavior

Inputting a number will target a specific day of a month.
- (next) 14(th)
- last 14(th)

#### Alternative behavior

This behavior can also be customized so that `next` strictly means "the next calendar
month".

- next 14th (equals to `14th next month`)
- last 14th (equals to `14th last month`)
- 14th (closest 14th in the future, same as `next 14th` in default mode)

### Days on relative month

#### Default behavior

As usual, `next` means the closest occurrence in the future, and `last` means the
opposite.

- (next) 14th April
- last 14 April

#### Alternative behavior

Using a parameter, this behavior can be altered to refer to the next or previous calendar
year.

- next 14th April (equals to `14th April next year`)
- last 14th April (equals to `14th April last year`)
- 14th April (closest 14th April in the future, same as `next 14th April` in default mode)

### Dates on relative intervals

It is possible to query a specific date interval and perform a "subquery" on it. An
interval can be a certain week, month, or year relative to today. Supported intervals are
`week`, `month`, and `year`, then a day of the week, a day of the month, or a specific
day/month of a year can be selected, respectively.

- wednesday this week (regardless of the current day, only the relative week counts)
- friday 2 weeks ago
- tuesday in 2 weeks
- 14th next month
- 17th in two months
- 10 this month
- 1st May this year
- 10th July in two years
