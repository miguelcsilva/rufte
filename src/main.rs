extern crate glob;
use std::fs::read_to_string;
mod path;
mod statistics;
use path::get_file_paths;
use statistics::get_number_of_lines;

const SOURCE_FOLDER: &str = "./src";

fn main() {
    let paths = get_file_paths(SOURCE_FOLDER);
    for path in paths {
        let contents = read_to_string(&path);
        println!("{}: {}", &path.display(), get_number_of_lines(&contents.unwrap()))
    }
}
