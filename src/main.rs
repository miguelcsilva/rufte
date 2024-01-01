extern crate glob;
use std::fs::read_to_string;
mod path;
mod statistics;
use path::file_paths;
use statistics::number_of_lines;

const SOURCE_FOLDER: &str = "./src";

fn main() {
    let paths = file_paths(SOURCE_FOLDER);
    for path in paths {
        let contents = read_to_string(&path);
        println!(
            "{}: {}",
            &path.display(),
            number_of_lines(&contents.unwrap())
        )
    }
}
