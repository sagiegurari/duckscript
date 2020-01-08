use super::*;
use crate::sdk::std::on_error::on_error;
use crate::test;
use crate::test::{CommandValidation, ErrorCommand};

#[test]
fn common_functions() {
    test::test_common_command_functions(create(""));
}

#[test]
fn run_no_args() {
    test::run_script_and_validate(
        vec![create(""), on_error::create(""), Box::new(ErrorCommand {})],
        r#"
        test_error
        out = get_last_error_source
        "#,
        CommandValidation::Match("out".to_string(), "".to_string()),
    );
}
