use std::{env, fs, path::Path};

use anyhow::{anyhow, Result};
use git2::{Repository, Status};
use tbb_test::{for_each_code_block, rewrite, run_commands, with_doc, Mode};

fn main() -> Result<()> {
    match env::args().skip(1).next() {
        Some(mode) if &mode == "coverage" => generate_coverage(),
        Some(mode) if &mode == "update" => update_doc_examples(),
        _ => Err(anyhow!("Usage: tbb_test <coverage|update> files...")),
    }
}

fn generate_coverage() -> Result<()> {
    for path in get_md_arguments()? {
        with_doc(&path, |contents, date, db_path| {
            for_each_code_block(&contents, |code| {
                run_commands(code, Mode::Coverage, &db_path, &date)
                    .expect("Error running code coverage");
            })
        })?;
    }
    Ok(())
}

fn update_doc_examples() -> Result<()> {
    for path in get_valid_arguments()? {
        let new_contents = with_doc(&path, |contents, date, db_path| {
            rewrite(contents, |code| {
                run_commands(code, Mode::Run, db_path, date).unwrap()
            })
        })?;
        fs::write(path, new_contents)?;
    }
    Ok(())
}

/// Returns command line arguments that satisfy all three conditions below:
/// - end in .md
/// - represent files checked into git
/// - represent files that have no uncommited changes
fn get_valid_arguments() -> Result<Vec<String>> {
    let md_args = get_md_arguments()?;

    let repo = Repository::open(".")?;

    let md_args_with_status: Vec<(String, Status)> = md_args
        .into_iter()
        .map(|arg| {
            repo.status_file(Path::new(&arg))
                .and_then(|status| Ok((arg, status)))
        })
        .collect::<Result<_, _>>()?;

    let (current_md_args, modified_md_args): (Vec<(String, Status)>, Vec<(String, Status)>) =
        md_args_with_status
            .into_iter()
            .partition(|(_, status)| *status == Status::CURRENT);

    if !modified_md_args.is_empty() {
        eprintln!(
            "Warning: {} file(s) skipped because they have uncommitted changes",
            modified_md_args.len(),
        )
    }

    Ok(current_md_args.into_iter().map(|(arg, _)| arg).collect())
}

/// Returns command line arguments that end in .md
fn get_md_arguments() -> Result<Vec<String>> {
    let (md_args, non_md_args): (Vec<String>, Vec<String>) =
        env::args().skip(2).partition(|arg| arg.ends_with(".md"));

    if !non_md_args.is_empty() {
        eprintln!(
            "Warning: {} file(s) skipped because they do not end in '.md'",
            non_md_args.len(),
        )
    }

    Ok(md_args)
}
