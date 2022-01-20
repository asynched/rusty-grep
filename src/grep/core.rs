use colored::Colorize;
use regex::Regex;

fn read_source_file(filename: &str) -> String {
    std::fs::read_to_string(filename).expect("Couldn't read source file!")
}

fn compile_regex(pattern: &str) -> Regex {
    Regex::new(pattern).expect(
        "Couldn't compile source regex. Check if the regex you've passed was valid and try again.",
    )
}

fn test_and_print_line(line: &str, pattern: &Regex) {
    if pattern.is_match(line) {
        let matches = pattern
            .find_iter(line)
            .map(|digits| digits.as_str())
            .collect::<Vec<&str>>();

        for (idx, item) in pattern.split(line).enumerate() {
            print!("{}", item);

            match matches.get(idx) {
                Some(item) => print!("{}", item.red().bold()),
                None => (),
            }
        }

        print!("\n");
    }
}

pub fn main() {
    let args = std::env::args().skip(1).collect::<Vec<String>>();

    let pattern = compile_regex(&args[0]);
    let file_contents = read_source_file(&args[1]);

    for line in file_contents.split('\n') {
        test_and_print_line(line, &pattern);
    }
}
