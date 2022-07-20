//! Tests that the example commands in /docs are accurate.

use std::{
    fs,
    path::{Path, PathBuf},
};

use tbb_test::{for_each_code_block, run_commands, Mode};

#[test]
fn test_doc_regression() -> anyhow::Result<()> {
    let mut docs_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    docs_path.push("docs");
    for doc in fs::read_dir(docs_path)? {
        let path = doc?.path();
        let contents = fs::read_to_string(path.clone())?;
        // Assume the first 10 chars are the date
        let date: String = contents.chars().take(10).collect();
        let db_path = path.to_string_lossy() + ".sqlite3";
        for_each_code_block(&contents, |code| {
            let new_code = run_commands(code, Mode::Run, &db_path, &date);
            if new_code.is_err() {
                eprintln!("Command failed to run in file: {:?}", path.clone());
                eprintln!("Code:\n{code}");
            }
            assert_eq!(code, new_code.unwrap());
        });
        let db_path = Path::new(db_path.as_ref());
        if db_path.exists() {
            fs::remove_file(Path::new(&db_path))?;
        }
    }
    Ok(())
}
