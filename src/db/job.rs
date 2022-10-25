use anyhow::Result;
use rusqlite::{named_params, Connection};

use super::common::{
    rank::{pre_insert, select_max_rank},
    verify_unique,
};

pub struct Job {
    pub name: String,
    pub rank: i64,
    id: i64,
}

pub const INFLOW_JOB_ID: i64 = 0;

pub fn init(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS jobs (
            job_id INTEGER NOT NULL PRIMARY KEY,
            name   TEXT NOT NULL COLLATE NOCASE UNIQUE,
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

impl super::Db {
    pub fn insert_job(&mut self, name: String) -> Result<()> {
        let conn = self.get_conn()?.transaction()?;
        let new_rank = select_max_rank(&conn, "jobs")? + 1;
        insert(&conn, name, new_rank)?;
        conn.commit()?;
        Ok(())
    }

    pub fn select_outflow_jobs(&mut self) -> Result<Vec<Job>> {
        let conn = self.get_conn()?.transaction()?;
        let jobs = select_outflow_jobs(&conn)?;
        conn.commit()?;
        Ok(jobs)
    }
}

fn insert(conn: &Connection, name: String, rank: i64) -> Result<Job> {
    verify_unique(conn, "name", name.clone(), "jobs")?;
    pre_insert(conn, rank, "jobs")?;
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

fn select_outflow_jobs(conn: &Connection) -> Result<Vec<Job>> {
    conn.prepare(
        "SELECT job_id, name, rank FROM jobs
        WHERE job_id != :1
        ORDER BY rank ASC",
    )?
    .query([INFLOW_JOB_ID])?
    .and_then(|row| {
        Ok(Job {
            id: row.get(0)?,
            name: row.get(1)?,
            rank: row.get(2)?,
        })
    })
    .collect()
}
