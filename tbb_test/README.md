# tbb_test

Tbb_test is a tool for processing tbb's markdown documentation. It can parse and execute example commands found in code blocks.

The main tbb repo uses tbb_test as a library to verify that the examples in the documentation run without errors and produce the expected output. The tbb_test repo can be run as a binary to overwrite the expected output in a markdown file with the actual output.

## Usage

```console
$ tbb_test files...
```

This will update the examples in the specified files. Files will only be modified if they have no uncommited changes.
