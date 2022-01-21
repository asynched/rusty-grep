use regex::Regex;

pub fn compile_regex(pattern: &str) -> Regex {
    Regex::new(pattern).expect(
        "Couldn't compile source regex. Check if the regex you've passed was valid and try again.",
    )
}
