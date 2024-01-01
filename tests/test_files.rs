use rufte::path::get_file_paths;
use std::env;
use std::fs::{create_dir_all, remove_dir_all, File};
use std::io::Write;

#[test]
fn test_get_files() {
    let tmp_dir = String::from(env!("CARGO_TARGET_TMPDIR")) + "/test_get_files";
    remove_dir_all(&tmp_dir).ok();
    let file_1_path = String::from(&tmp_dir) + "/f1.txt";
    let subfolder_path = String::from(&tmp_dir) + "/folder";
    let file_2_path = subfolder_path.to_owned() + "/f2";
    let file_3_path = subfolder_path.to_owned() + "/.ignore_file";

    create_dir_all(&tmp_dir).unwrap();
    File::create(&file_1_path).unwrap().write(b"test1").unwrap();
    create_dir_all(&subfolder_path).unwrap();
    File::create(&file_2_path).unwrap().write(b"test2").unwrap();
    File::create(&file_3_path)
        .unwrap()
        .write(b"ignore_me")
        .unwrap();

    let files: Vec<String> = get_file_paths(&tmp_dir)
        .iter()
        .map(|f| f.to_str().unwrap().to_string())
        .collect();
    let expected_files = vec![file_1_path, file_2_path];
    assert_eq!(files, expected_files);
}
