//! Tests that the example commands in /docs are accurate.

use std::{fs, path::PathBuf};

use tbb_test::{for_each_code_block, run_commands, with_doc, Mode};

#[test]
fn test_doc_regression() -> anyhow::Result<()> {
    let run_coverage = should_run_coverage();
    for doc in fs::read_dir(docs_path())? {
        let path = doc?.path();
        let path = path.to_str().expect("doc path is not a string");
        with_doc(path, |contents, date, db_path| {
            for_each_code_block(contents, |code| {
                let new_code = run_commands(code, Mode::Run, &db_path, &date);
                if new_code.is_err() {
                    eprintln!("Command failed to run in file: {path}");
                    eprintln!("Code:\n{code}");
                }
                assert_eq!(code, new_code.unwrap());
                if run_coverage {
                    // Repeat the command separately if coverage needs to be generated.
                    // We have to run separately because running with code coverage
                    // will write an error message to stdout when it finds a process
                    // that exits with an error. So when we try and test a command that
                    // intentionally results in an error, our test will not pass, as
                    // the output will contain an unexpected extra error message.
                    run_commands(code, Mode::Coverage, db_path, date)
                        .expect("Error running code coverage");
                }
            });
        })?;
    }
    Ok(())
}

fn should_run_coverage() -> bool {
    match std::env::var("TBB_COVERAGE") {
        Ok(s) => s == "1",
        Err(_) => false,
    }
}

fn docs_path() -> PathBuf {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("docs");
    path
}
