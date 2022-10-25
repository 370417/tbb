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

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct YearMonth {
    pub year: i64,
    /// 0-indexed
    pub month: u8,
}

impl YearMonth {
    pub fn to_int(self) -> i64 {
        self.year * 12 + self.month as i64
    }

    pub fn from_int(int: i64) -> YearMonth {
        let month = ((int % 12) + 12) % 12;
        let year = (int - month) / 12;
        YearMonth {
            year,
            month: month as u8,
        }
    }
}

#[cfg(test)]
mod tests {
    use proptest::prelude::*;

    use super::*;

    proptest! {
        #[test]
        fn month_same_after_roundtrip(month_int: i64) {
            prop_assert_eq!(month_int, YearMonth::from_int(month_int).to_int());
        }

        #[test]
        fn month_preserves_order(month_int1: i64, month_int2: i64) {
            let month1 = YearMonth::from_int(month_int1);
            let month2 = YearMonth::from_int(month_int2);
            prop_assert_eq!(month_int1.cmp(&month_int2), month1.cmp(&month2));
        }
    }
}
