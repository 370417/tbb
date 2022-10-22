use anyhow::Result;
use rusqlite::{named_params, Connection};

use super::common::{
    rank::{pre_insert, select_max_rank},
    verify_unique,
};

#[allow(dead_code)]
pub struct Account {
    pub name: String,
    pub rank: i64,
    id: i64,
}

pub(super) fn init(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS accounts (
            account_id INTEGER NOT NULL PRIMARY KEY,
            name       TEXT NOT NULL COLLATE NOCASE UNIQUE,
            rank       INTEGER NOT NULL
        )",
        [],
    )?;
    Ok(())
}

impl super::Db {
    pub fn insert_account(&mut self, name: String) -> Result<()> {
        let conn = self.get_conn()?.transaction()?;
        let new_rank = select_max_rank(&conn, "accounts")? + 1;
        insert(&conn, name, new_rank)?;
        conn.commit()?;
        Ok(())
    }
}

fn insert(conn: &Connection, name: String, rank: i64) -> Result<Account> {
    verify_unique(conn, "name", name.clone(), "accounts")?;
    pre_insert(conn, rank, "accounts")?;
    conn.execute(
        "INSERT INTO accounts (name, rank) VALUES (:name, :rank)",
        named_params! {
            ":name": name,
            ":rank": rank,
        },
    )?;
    let id = conn.last_insert_rowid();
    Ok(Account { id, name, rank })
}
