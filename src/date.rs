use anyhow::Result;
use time::{macros::format_description, Date, OffsetDateTime};

/// Get the current local date, or get the date from $TBB_DEFAULT_DATE if it exsits.
pub fn init_date() -> Result<Date> {
    std::env::var("TBB_DEFAULT_DATE")
        .map_err(anyhow::Error::from)
        .and_then(parse_numerical_date)
        .or_else(|_| get_today())
}

fn get_today() -> Result<Date> {
    OffsetDateTime::now_local()
        .map(OffsetDateTime::date)
        .map_err(anyhow::Error::from)
}

pub fn parse_numerical_date<StrRef: AsRef<str>>(date_str: StrRef) -> Result<Date> {
    let format = format_description!("[year]-[month]-[day]");
    Date::parse(date_str.as_ref(), &format).map_err(anyhow::Error::from)
}

pub fn format_month_year(date: &Date) -> Result<String> {
    let format = format_description!("[month repr:short] [year]");
    date.format(&format).map_err(anyhow::Error::from)
}
