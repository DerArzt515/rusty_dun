use std::fs;

pub fn read_file_to_str(file_loc: &str) -> String {
    fs::read_to_string(file_loc).expect("Something went wrong reading the file")
}
