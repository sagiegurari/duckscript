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
