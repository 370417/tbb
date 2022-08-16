//! Tests that the example commands in /docs are accurate.

use std::{fs, path::PathBuf};

use tbb_test::{for_each_code_block, run_commands, with_doc, Mode};

#[test]
fn test_doc_regression() -> anyhow::Result<()> {
    for doc in fs::read_dir(docs_path())? {
        let path = doc?.path();
        let path = path.to_str().expect("doc path is not a string");
        with_doc(path, |contents, date, db_path| {
            for_each_code_block(&contents, |code| {
                let new_code = run_commands(code, Mode::Run, &db_path, &date);
                if new_code.is_err() {
                    eprintln!("Command failed to run in file: {path}");
                    eprintln!("Code:\n{code}");
                }
                assert_eq!(code, new_code.unwrap());
            });
        })?;
    }
    Ok(())
}

fn docs_path() -> PathBuf {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("docs");
    path
}
