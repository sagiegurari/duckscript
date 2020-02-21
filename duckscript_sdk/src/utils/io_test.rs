use super::*;
use std::path::Path;

#[test]
fn read_text_file_found() {
    let text = read_text_file("./Cargo.toml");

    assert!(text.unwrap().contains("duckscript"));
}

#[test]
fn read_text_file_not_found() {
    let text = read_text_file("./Cargo.toml2");

    assert!(text.is_err());
}

#[test]
fn write_text_file_valid() {
    let result = write_text_file("./target/_duckscript/temp/test/test.txt", "test file");

    assert!(result.is_ok());

    let text = read_text_file("./target/_duckscript/temp/test/test.txt").unwrap();

    assert_eq!(text, "test file");
}

#[test]
fn write_to_text_file_exists() {
    let mut result = write_text_file(
        "./target/_duckscript/temp/test/write_to_text_file_exists.txt",
        "test file",
    );

    assert!(result.is_ok());

    result = write_to_text_file(
        "./target/_duckscript/temp/test/write_to_text_file_exists.txt",
        "\ntest file",
        true,
    );

    assert!(result.is_ok());

    let text =
        read_text_file("./target/_duckscript/temp/test/write_to_text_file_exists.txt").unwrap();

    assert_eq!(text, "test file\ntest file");
}

#[test]
fn write_to_text_file_not_exists() {
    let mut result = write_to_text_file(
        "./target/_duckscript/temp/test/write_to_text_file_not_exists.txt",
        "test file",
        true,
    );

    assert!(result.is_ok());

    result = write_to_text_file(
        "./target/_duckscript/temp/test/write_to_text_file_not_exists.txt",
        "\ntest file",
        true,
    );

    assert!(result.is_ok());

    let text =
        read_text_file("./target/_duckscript/temp/test/write_to_text_file_not_exists.txt").unwrap();

    assert_eq!(text, "test file\ntest file");
}

#[test]
fn create_empty_file_not_exists() {
    let path = "./target/_duckscript/temp/create_empty_file_not_exists/file.txt";
    let file_path = Path::new(path);
    assert!(!file_path.exists());

    let result = create_empty_file(path);

    assert!(result.is_ok());
    assert!(file_path.exists());
}

#[test]
fn create_empty_file_exists() {
    let path = "./target/_duckscript/temp/create_empty_file_exists/file.txt";
    let file_path = Path::new(path);
    assert!(!file_path.exists());

    let result = create_empty_file(path);

    assert!(result.is_ok());
    assert!(file_path.exists());

    let result = create_empty_file(path);
    assert!(result.is_ok());
}
