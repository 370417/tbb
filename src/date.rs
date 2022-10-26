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

/// YearMonth exists so that the year and month can be combined
/// into one column in the database. We do this to simplify
/// comparisons between dates.
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
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

    #[test]
    fn jan_0_is_origin() {
        let origin = YearMonth { year: 0, month: 0 };
        assert_eq!(origin.to_int(), 0);
    }

    proptest! {
        #[test]
        fn year_month_same_after_roundtrip(int: i64) {
            prop_assert_eq!(int, YearMonth::from_int(int).to_int());
        }

        #[test]
        fn year_month_from_int_preserves_order(int1: i64, int2: i64) {
            let year_month1 = YearMonth::from_int(int1);
            let year_month2 = YearMonth::from_int(int2);
            prop_assert_eq!(int1.cmp(&int2), year_month1.cmp(&year_month2));
        }
    }
}
