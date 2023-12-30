extern crate glob;
use std::fs::read_to_string;
mod file;
use file::get_files;

const SOURCE_FOLDER:  &str = "./src";

fn main() {
    let files = get_files(SOURCE_FOLDER);
    for file in files {
        let contents = read_to_string(file);
        println!("{}", contents.unwrap())
    };
}
