mod date;

use anyhow::Result;
use clap::{Parser, Subcommand};

fn main() -> Result<()> {
    let args = Args::parse();
    let today = date::init_date()?;

    match args.command {
        Command::Status => println!("[ {} ]", date::format_month_year(&today)?),
    }

    Ok(())
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
}
