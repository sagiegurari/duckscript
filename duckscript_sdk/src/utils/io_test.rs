use super::*;

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
    let result = write_text_file("./target/temp/test/test.txt", "test file");

    assert!(result.is_ok());

    let text = read_text_file("./target/temp/test/test.txt").unwrap();

    assert_eq!(text, "test file");
}
