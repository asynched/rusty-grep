use super::files;
use super::regexp;
use super::stdin;
use super::stdout;

use clap::Parser;

#[derive(Parser, Debug)]
struct Arguments {
    /// Filename to search within the file system
    #[clap(short, long, default_value = "")]
    filename: String,

    /// Regex to search within the file
    #[clap(short, long)]
    pattern: String,

    /// Optional value to show line numbers or not
    #[clap(short, long)]
    line_numbers: bool,
}

/// Executes the application, it is self-contained so it doesn't take any arguments
///
/// # Example
///
/// ```
/// grep::core::main();
/// ```
pub fn main() {
    let args = Arguments::parse();

    let pattern = regexp::compile_regex(&args.pattern);

    // Read from filename or read directly from piped stdin
    // if a filename wasn't provided
    let contents = if args.filename.len() > 0 {
        files::read_source_file(&args.filename)
    } else {
        stdin::read_from_stdin()
    };

    // Print line number if the flag was set in the arguments
    if args.line_numbers {
        for (number, line) in contents.split('\n').enumerate() {
            stdout::print_line_with_number(line, number + 1, &pattern);
        }
        return;
    }

    // Default behavior is to read the content and not print it's lines
    for line in contents.split('\n') {
        stdout::print_line(line, &pattern);
    }
}
