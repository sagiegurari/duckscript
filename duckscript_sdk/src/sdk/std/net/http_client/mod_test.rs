use super::*;
use crate::test;
use crate::test::{CommandValidation, SetCommand};

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
    let result = io::delete_file(file);
    assert!(result.is_ok());

    test::run_script_and_validate(
        vec![create("")],
        "out = http_client -O ./target/_duckscript/http_client/page.html https://www.rust-lang.org/",
        CommandValidation::Ignore
    );

    let read_result = io::read_text_file(file);
    assert!(read_result.is_ok());

    let text = read_result.unwrap();
    assert!(text.contains("Rust"));
}

#[test]
fn run_post() {
    test::run_script_and_validate(
        vec![create(""), Box::new(SetCommand {})],
        r#"
        payload = test_set {\"login\":\"login\",\"password\":\"password\"}
        out = http_client --method=HTTP-POST --post-data=${payload} https://reqbin.com/echo/post/json
        "#,
        CommandValidation::Contains("out".to_string(), "success".to_string()),
    );
}
