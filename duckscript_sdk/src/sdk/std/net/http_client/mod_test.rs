use super::*;
use crate::test;
use crate::test::{CommandValidation, SetCommand};
use fsio::file::{delete_ignore_error, read_text_file};

#[test]
fn common_functions() {
    test::test_common_command_functions(create(""));
}

#[test]
fn run_no_args() {
    test::run_script_and_error(vec![create("")], "out = http_client", "out");
}

#[test]
fn run_get() {
    test::run_script_and_validate(
        vec![create("")],
        "out = http_client https://www.rust-lang.org/",
        CommandValidation::Contains("out".to_string(), "Rust".to_string()),
    );
}

#[test]
fn run_get_to_file() {
    let file = "./target/_duckscript/http_client/page.html";
    let result = delete_ignore_error(file);
    assert!(result);

    test::run_script_and_validate(
        vec![create("")],
        "out = http_client --output-file ./target/_duckscript/http_client/page.html https://www.rust-lang.org/",
        CommandValidation::PositiveNumber("out".to_string())
    );

    let read_result = read_text_file(file);
    assert!(read_result.is_ok());

    let text = read_result.unwrap();
    assert!(text.contains("Rust"));
}

#[cfg(target_os = "linux")]
#[test]
fn run_post() {
    test::run_script_and_validate(
        vec![create(""), Box::new(SetCommand {})],
        r#"
        payload = test_set {\"login\":\"login\",\"password\":\"password\"}
        out = http_client --method POST --payload ${payload} https://postman-echo.com/post
        "#,
        CommandValidation::Contains("out".to_string(), "password".to_string()),
    );
}

#[test]
fn run_invalid_url() {
    test::run_script_and_error(vec![create("")], "out = http_client invalid_url", "out");
}
