use std::env::VarError;

use anyhow::{anyhow, Result};
use chrono::prelude::*;

/// Get the current local date, or get the date from $TBB_DEFAULT_DATE if it exsits.
pub fn init_date() -> Result<NaiveDate> {
    match std::env::var("TBB_DEFAULT_DATE") {
        Ok(date_str) => Ok(NaiveDate::parse_from_str(&date_str, "%Y-%m-%d")?),
        Err(VarError::NotPresent) => Ok(Local::now().date_naive()),
        Err(VarError::NotUnicode(_)) => Err(anyhow!(
            "Cannot read TBB_DEFAULT_DATE environment variable: contents are not Unicode"
        )),
    }
}

pub fn format_month_year(date: &NaiveDate) -> String {
    date.format("%b %Y").to_string()
}
