# Rusty GREP

A rewrite of GREP using Rust. 🦀

## Usage

The usage is very basic, there are mostly 2 arguments, the regex pattern and the source file to find the items in.

| argument    | type   |
| ----------- | ------ |
| pattern     | string |
| source file | string |

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
- [ ] Implement colored output option for the end user
- [ ] Implement logic for piped output (such as `cat $SOURCE | grep ...`)
- [ ] Implement line numbers
- [ ] Implement reversed regex

## Author

| ![Eder Lima](https://github.com/asynched.png?size=100) |
| ------------------------------------------------------ |
| [Eder Lima](https://github.com/asynched)               |
