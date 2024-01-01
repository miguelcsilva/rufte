extern crate glob;
use glob::glob;
use std::path::PathBuf;

pub fn file_paths(directory: &str) -> Vec<PathBuf> {
    let search_pattern = directory.to_string() + "/**/[!.]*";
    let paths = glob(&search_pattern).unwrap();
    paths
        .map(|path| path.unwrap())
        .filter(|path| path.is_file())
        .collect()
}
