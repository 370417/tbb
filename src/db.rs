mod account;
mod common;
mod job;

use anyhow::Result;
use rusqlite::Connection;

pub struct Db {
    state: DbState,
}

// The db connection is created lazily
enum DbState {
    Path(String),
    Connection(Connection),
}

impl Db {
    pub fn create(db_path: String) -> Db {
        Db {
            state: DbState::Path(db_path),
        }
    }

    #[allow(dead_code)]
    pub fn create_in_memory() -> Result<Db> {
        Ok(Db {
            state: DbState::Connection(init_db(Connection::open_in_memory()?)?),
        })
    }

    fn get_conn(&mut self) -> Result<&mut Connection> {
        let state = std::mem::take(&mut self.state);

        self.state = DbState::Connection(match state {
            DbState::Connection(conn) => conn,
            DbState::Path(path) => init_db(Connection::open(&path)?)?,
        });

        match &mut self.state {
            DbState::Connection(conn) => Ok(conn),
            DbState::Path(_) => unreachable!(),
        }
    }
}

fn init_db(conn: Connection) -> Result<Connection> {
    account::init(&conn)?;
    job::init(&conn)?;
    Ok(conn)
}

// Implement Default to allow std::mem::take to work with DbState
impl Default for DbState {
    fn default() -> Self {
        DbState::Path(String::new())
    }
}
