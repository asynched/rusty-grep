use std::io::{self, BufRead};

pub fn read_from_stdin() -> String {
    let stdin = io::stdin();

    // FIXME: This has a bug for when the user doesn't provide a filename
    // and the process hasn't received any piped data.
    stdin
        .lock()
        .lines()
        .map(|item| {
            item.expect("Couldn't read from stdin, have you piped any input to the program?")
        })
        .collect::<Vec<String>>()
        .join("\n")
}
