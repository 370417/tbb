use rusqlite::{named_params, Connection, Result};

use super::Db;

#[allow(dead_code)]
pub struct Job {
    pub name: String,
    pub rank: i64,
    id: i64,
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
        conn.commit()?;
        Ok(())
    }

    pub fn select_outflow_jobs(&mut self) -> anyhow::Result<Vec<Job>> {
        let conn = self.get_conn()?.transaction()?;
        let jobs = select_outflow_jobs(&conn)?;
        conn.commit()?;
        Ok(jobs)
    }
}

/// Returns 0 if no rows are found
fn select_max_rank(conn: &Connection) -> Result<i64> {
    conn.query_row("SELECT COALESCE(MAX(rank), 0) FROM jobs", [], |row| {
        row.get(0)
    })
}

fn insert(conn: &Connection, name: String, rank: i64) -> Result<Job> {
    pre_insert(conn, rank)?;
    conn.execute(
        "INSERT INTO jobs (name, rank) VALUES (:name, :rank)",
        named_params! {
            ":name": name,
            ":rank": rank,
        },
    )?;
    let id = conn.last_insert_rowid();
    Ok(Job { id, name, rank })
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

fn select_outflow_jobs(conn: &Connection) -> Result<Vec<Job>> {
    conn.prepare(
        "SELECT job_id, name, rank FROM jobs
        WHERE job_id != :1
        ORDER BY name ASC",
    )?
    .query_map([INFLOW_JOB_ID], |row| {
        Ok(Job {
            id: row.get(0)?,
            name: row.get(1)?,
            rank: row.get(2)?,
        })
    })?
    .collect()
}
