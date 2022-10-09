# tbb_test

Tbb_test is a tool for processing tbb's markdown documentation. It can parse and execute example commands found in code blocks.

The main tbb repo uses tbb_test as a library to verify that the examples in the documentation run without errors and produce the expected output. The tbb_test repo can be run as a binary to overwrite the expected output in a markdown file with the actual output. To make running the binary easier, there's a shell script in the main tbb repo called [rerun_doc_examples.sh](../rerun_doc_examples.sh).

## Usage

```console
$ tbb_test <coverage|update> files...
```

Running with the subcommand `coverage` will run [cargo-llvm-cov](https://github.com/taiki-e/cargo-llvm-cov) without output. To generate a report, run `cargo llvm-cov report --lcov`. Cargo-llvm-cov needs to be installed.

Running with the subcommand `update` will update the examples in the docs. Files will only be modified if they have no uncommited changes.
