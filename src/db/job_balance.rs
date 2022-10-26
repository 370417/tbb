use anyhow::Result;
use rusqlite::{Connection, OptionalExtension};

use crate::date::YearMonth;

pub struct JobBalance {
    pub year_month: YearMonth,
    pub job_id: i64,
    /// Running balance in cents including transactions made in previous months.
    pub balance: i64,
    /// Cents assigned to this job during the current month only.
    pub assigned: i64,
    /// Cents of activity during the current month only.
    /// Spending X cents results in negative X activity.
    pub activity: i64,
}

impl JobBalance {
    fn new(year_month: YearMonth, job_id: i64) -> JobBalance {
        JobBalance {
            year_month,
            job_id,
            balance: 0,
            assigned: 0,
            activity: 0,
        }
    }

    /// When carrying a balance over to a new month, the balance stays the same,
    /// but the assigned and activity amounts for the month get reset to 0.
    fn with_year_month(self, year_month: YearMonth) -> JobBalance {
        if year_month == self.year_month {
            self
        } else {
            JobBalance {
                year_month,
                job_id: self.job_id,
                balance: self.balance,
                assigned: 0,
                activity: 0,
            }
        }
    }
}

pub fn init(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS job_balances (
            year_month  INTEGER NOT NULL,
            job_id      INTEGER NOT NULL REFERENCES jobs,
            balance     INTEGER NOT NULL,
            assigned    INTEGER NOT NULL,
            activity    INTEGER NOT NULL,
            PRIMARY KEY (year_month, job_id)
        )",
        [],
    )?;
    Ok(())
}

/// Select a month's running balance.
///
/// If the month has never had any transactions for this job,
/// this will return a JobBalance with the desired month and
/// will pull the correct balance from a past month. If there
/// is no past data, the balance will be 0.
fn select(conn: &Connection, year_month: YearMonth, job_id: i64) -> Result<JobBalance> {
    Ok(select_closest(conn, year_month, job_id)?
        .map(|balance| balance.with_year_month(year_month))
        .unwrap_or(JobBalance::new(year_month, job_id)))
}

/// Select the row that has the correct running balance for the desired month.
///
/// If the desired month has no row in the table, this will try to select the closest
/// month in the past. So the date of the returned JobBalance can differ from the date
/// passed in as an argument.
fn select_closest(
    conn: &Connection,
    year_month: YearMonth,
    job_id: i64,
) -> rusqlite::Result<Option<JobBalance>> {
    conn.query_row(
        "SELECT year_month, balance, assigned, activity FROM job_balances
        WHERE year_month <= :1 AND job_id == :2
        ORDER BY year_month DESC
        LIMIT 1",
        [year_month.to_int(), job_id],
        |row| {
            Ok(JobBalance {
                year_month: YearMonth::from_int(row.get(0)?),
                balance: row.get(1)?,
                assigned: row.get(2)?,
                activity: row.get(3)?,
                job_id,
            })
        },
    )
    .optional()
}

fn update_balance(
    conn: &Connection,
    year_month: YearMonth,
    job_id: i64,
    assigned_delta: i64,
    activity_delta: i64,
) -> Result<()> {
    let job_balance = select(conn, year_month, job_id)?;
    // Update specified month
    conn.execute(
        "INSERT OR REPLACE INTO job_balances (year_month, job_id, balance, assigned, activity)
        VALUES (:1, :2, :3, :4, :5)",
        [
            job_balance.year_month.to_int(),
            job_balance.job_id,
            job_balance.balance + assigned_delta + activity_delta,
            job_balance.assigned + assigned_delta,
            job_balance.activity + activity_delta,
        ],
    )?;
    // Update future months
    conn.execute(
        "UPDATE job_balances SET
            balance = balance + :3 + :4
        WHERE year_month > :1 AND job_id == :2",
        [year_month.to_int(), job_id, assigned_delta, activity_delta],
    )?;
    Ok(())
}
