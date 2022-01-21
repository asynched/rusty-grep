pub fn read_source_file(filename: &str) -> String {
    std::fs::read_to_string(filename).expect("Couldn't read source file!")
}
