use rusqlite::Connection;

pub mod rank {
    use super::*;

    /// Returns 0 if no rows are found
    pub fn select_max_rank(conn: &Connection, table: &str) -> rusqlite::Result<i64> {
        conn.query_row(
            &format!("SELECT COALESCE(MAX(rank), 0) FROM {table}"),
            [],
            |row| row.get(0),
        )
    }

    /// Update ranks of other rows before inserting a new row into the category
    pub fn pre_insert(conn: &Connection, rank: i64, table: &str) -> rusqlite::Result<()> {
        conn.execute(
            &format!("UPDATE {table} SET rank = rank + 1 WHERE rank >= :1"),
            [rank],
        )
        .map(std::mem::drop)
    }
}

pub fn verify_unique(
    conn: &Connection,
    key: &str,
    value: String,
    table: &str,
) -> anyhow::Result<()> {
    let count: i64 = conn.query_row(
        &format!("SELECT COUNT(*) FROM {table} WHERE {key} == :1"),
        [value],
        |row| row.get(0),
    )?;
    match count {
        0 => Ok(()),
        _ => Err(anyhow::anyhow!("{key} is not unique in {table}")),
    }
}
