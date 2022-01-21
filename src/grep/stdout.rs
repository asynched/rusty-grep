use colored::Colorize;
use regex::Regex;

fn parse_matches(line: &str, pattern: &Regex) -> Vec<String> {
    pattern
        .find_iter(line)
        .map(|digits| digits.as_str().to_owned())
        .collect::<Vec<String>>()
}

pub fn print_line(line: &str, pattern: &Regex) {
    if pattern.is_match(line) {
        let matches = parse_matches(line, pattern);

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

pub fn print_line_with_number(line: &str, number: usize, pattern: &Regex) {
    if pattern.is_match(line) {
        let matches = parse_matches(line, pattern);

        let line_number = format!("{}: ", number);
        print!("{}", line_number.green().bold());

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
