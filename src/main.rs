mod date;
mod db;
mod status;

use std::env::VarError;

use anyhow::{anyhow, Result};
use clap::{Parser, Subcommand};
use db::Db;
use time::Date;

fn main() -> Result<()> {
    let args = Args::parse();
    let today = date::init_date()?;

    let db_path = args.db.ok_or_else(|| std::env::var("TBB_DB_FILE"));
    let db_path = handle_db_path_err(db_path)?;
    let mut db = Db::create(db_path);

    args.command.execute(&mut db, today)?;

    Ok(())
}

fn handle_db_path_err(db_path: Result<String, Result<String, VarError>>) -> Result<String> {
    match db_path {
        Ok(str) | Err(Ok(str)) => Ok(str),
        Err(Err(VarError::NotPresent)) => Err(anyhow!("At least one of the following is required:\n  --db command-line option\n  TBB_DB_FILE environment variable")),
        Err(Err(VarError::NotUnicode(_))) => Err(anyhow!("Cannot read TBB_DB_FILE environment variable: contents are not Unicode")),
    }
}

#[derive(Parser)]
#[command()]
struct Args {
    #[clap(subcommand)]
    pub command: Command,
    /// Path to sqlite file; defaults to $TBB_DB_FILE
    #[arg(long)]
    pub db: Option<String>,
}

#[derive(Subcommand)]
enum Command {
    /// Show one month's budget
    Status,
    #[clap(subcommand)]
    Account(AccountCommand),
    #[clap(subcommand)]
    Job(JobCommand),
}

#[derive(Subcommand)]
enum AccountCommand {
    Add { account_name: String },
}

#[derive(Subcommand)]
enum JobCommand {
    Add { job_name: String },
}

impl Command {
    fn execute(&self, db: &mut Db, today: Date) -> Result<()> {
        match self {
            Self::Status => status::print_status(db, today)?,
            Self::Account(account_command) => account_command.execute(db)?,
            Self::Job(job_command) => job_command.execute(db)?,
        }
        Ok(())
    }
}

impl AccountCommand {
    fn execute(&self, db: &mut Db) -> Result<()> {
        match self {
            Self::Add { account_name } => db.insert_account(account_name.clone()),
        }
    }
}

impl JobCommand {
    fn execute(&self, db: &mut Db) -> Result<()> {
        match self {
            Self::Add { job_name } => db.insert_job(job_name.clone()),
        }
    }
}
