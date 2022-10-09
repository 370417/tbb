mod eval;
mod markdown;

use std::{fs, path::Path};

pub use eval::{run_commands, Mode};
pub use markdown::{for_each_code_block, rewrite};

/// Provides a function with
///
/// - the contents of a doc file
/// - the date in the file's header
/// - a path for an associated db
///
/// (as parameters in that order)
///
/// and deletes the db after the function runs
pub fn with_doc<F, T>(path: &str, fun: F) -> anyhow::Result<T>
where
    F: Fn(&str, &str, &str) -> T,
{
    let contents = fs::read_to_string(path)?;
    let date: String = contents.chars().take(10).collect();
    let db_path = path.to_owned() + ".sqlite3";
    let output = fun(&contents, &date, &db_path);
    let db_path = Path::new(&db_path);
    if db_path.exists() {
        fs::remove_file(db_path)?;
    }
    Ok(output)
}
