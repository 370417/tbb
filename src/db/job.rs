use rusqlite::{named_params, Connection, Result};

use super::Db;

pub struct Job {
    _id: i64,
    _name: String,
    _rank: i64,
}

pub const INFLOW_JOB_ID: i64 = 0;

pub(super) fn init(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS jobs (
            job_id INTEGER NOT NULL PRIMARY KEY,
            name   TEXT NOT NULL COLLATE NOCASE,
            rank   INTEGER NOT NULL
        )",
        [],
    )?;
    conn.execute(
        "INSERT OR IGNORE INTO jobs (job_id, name, rank)
        VALUES (:1, '', 0)",
        [INFLOW_JOB_ID],
    )?;
    Ok(())
}

impl Db {
    pub fn insert_job(&mut self, name: String) -> anyhow::Result<()> {
        let conn = self.get_conn()?.transaction()?;
        let new_rank = select_max_rank(&conn)? + 1;
        insert(&conn, name, new_rank)?;
        Ok(())
    }

    pub fn _select_all_jobs(&mut self) -> anyhow::Result<Vec<Job>> {
        let conn = self.get_conn()?.transaction()?;
        _select_all_jobs(&conn).map_err(anyhow::Error::from)
    }
}

/// Returns 0 if no rows are found
fn select_max_rank(conn: &Connection) -> Result<i64> {
    conn.query_row("SELECT COALESCE(MAX(rank), 0) FROM jobs", [], |row| {
        row.get(0)
    })
}

fn insert(conn: &Connection, _name: String, _rank: i64) -> Result<Job> {
    pre_insert(conn, _rank)?;
    conn.execute(
        "INSERT INTO jobs (name, rank) VALUES (:name, :rank)",
        named_params! {
            ":name": _name,
            ":rank": _rank,
        },
    )?;
    let _id = conn.last_insert_rowid();
    Ok(Job { _id, _name, _rank })
}

/// Update ranks of other jobs before inserting a new job into the category
fn pre_insert(conn: &Connection, rank: i64) -> Result<()> {
    conn.execute(
        "UPDATE jobs SET rank = rank + 1
        WHERE rank >= :1",
        [rank],
    )?;
    Ok(())
}

fn _select_all_jobs(conn: &Connection) -> Result<Vec<Job>> {
    conn.prepare(
        "SELECT job_id, name, rank, category_id FROM jobs
        ORDER BY name ASC",
    )?
    .query_map([], |row| {
        Ok(Job {
            _id: row.get(0)?,
            _name: row.get(1)?,
            _rank: row.get(2)?,
        })
    })?
    .collect()
}
