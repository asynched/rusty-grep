# Rusty GREP

A rewrite of GREP using Rust. 🦀

## Usage

The usage is very basic, there are mostly 2 arguments, the regex pattern and the source file to find the items in.

| argument       | short | description                                          |
| -------------- | ----- | ---------------------------------------------------- |
| --filename     | -f    | Filename to search within the file system (Optional) |
| --help         | -h    | Print help information                               |
| --line-numbers | -l    | Optional value to show line numbers or not           |
| --pattern      | -p    | Regex to search within the file                      |

### Usage example

```sh
$ $COMPILED $PATTERN $SOURCE_FILE
```

## Building

To build it from source, run the following command:

```sh
$ cargo build --release
```

## TODOs

- [x] Implement logic from reading from file
- [x] Implement colored highlight
- [x] Implement logic for piped output (such as `cat $SOURCE | grep ...`)
- [x] Implement line numbers

## Author

| ![Eder Lima](https://github.com/asynched.png?size=100) |
| ------------------------------------------------------ |
| [Eder Lima](https://github.com/asynched)               |
