use anyhow::Result;
use chrono::NaiveDate;

use crate::{date::format_month_year, db::Db};

pub fn print_status(db: &mut Db, today: NaiveDate) -> Result<()> {
    println!("[ {} ]", format_month_year(&today));
    for job in db.select_outflow_jobs()? {
        println!("{}", job.name);
    }
    Ok(())
}
