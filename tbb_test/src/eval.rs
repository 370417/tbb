//! Evaluates strings as if they were tbb commands typed in the shell

use std::process::Command;

use anyhow::anyhow;

/// Whether to run normally or to generate code coverage
#[derive(Clone, Copy)]
pub enum Mode {
    Run,
    Coverage,
}

impl Mode {
    fn command_fragment(self) -> &'static str {
        match self {
            Mode::Run => "cargo run -q --",
            Mode::Coverage => "cargo llvm-cov run --no-report --",
        }
    }
}

/// Input: `code` is a string of tbb commands with optional output.
/// Each tbb command must be preceded by a dollar sign
/// and must be only one line long.
/// Anything in the input that isn't a command gets ignored.
///
/// Date should be in yyyy-MM-dd format.
///
/// Output: commands from the input interleaved with each command's output.
///
/// In run mode, this function runs commands normally, withe cargo run.
///
/// In coverage mode, this function runs commands to generate coverage using cargo-llvm-cov.
/// To get the actual coverage report, `cargo llvm-cov --no-run --lcov` should be run separately
pub fn run_commands(
    code: &str,
    mode: Mode,
    db_filename: &str,
    date: &str,
) -> anyhow::Result<String> {
    let command_lines = code.lines().filter(|line| line.starts_with("$ tbb"));
    let mut net_output = String::new();
    for line in command_lines {
        let command = line.replacen("$ tbb", mode.command_fragment(), 1);
        let mut args = shell_words::split(&command)?;
        let command = args.remove(0);
        let output = Command::new(command)
            .env("TBB_DB_FILE", db_filename)
            .env("TBB_DEFAULT_DATE", date)
            .args(args)
            .output()?;
        net_output.push_str(line);
        net_output.push('\n');
        net_output.push_str(std::str::from_utf8(&output.stdout)?);
        net_output.push_str(std::str::from_utf8(&output.stderr)?);
        // Constraint: we only ever output to stdout xor stderr.
        // To support both at once, we'd need to watch the output as it arrives
        // and interleave it into one string.
        if !output.stdout.is_empty() && !output.stderr.is_empty() {
            return Err(anyhow!("stdout and stderr both exist\n{net_output}"));
        }
    }
    Ok(net_output)
}
