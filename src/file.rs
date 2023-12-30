extern crate glob;
use glob::glob;
use std::path::PathBuf;

pub fn get_files(directory: &str) -> Vec<PathBuf> {
    let search_pattern = directory.to_string() + "/**/[!.]*";
    let paths = glob(&search_pattern).unwrap();
    let file_paths: Vec<PathBuf> = paths
        .map(|x| x.unwrap())
        .filter(|file| file.is_file())
        .collect();
    file_paths
}
