use super::*;

#[test]
fn get_parent_directory_name_with_dir() {
    let directory = get_parent_directory_name("./dir/file.txt");

    assert_eq!(directory.unwrap(), "./dir");
}

#[test]
fn get_parent_directory_name_found() {
    let directory = get_parent_directory_name("./file.txt");

    assert_eq!(directory.unwrap(), ".");
}

#[test]
fn get_parent_directory_name_file_only() {
    let directory = get_parent_directory_name("file.txt");

    assert!(directory.is_none());
}

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
