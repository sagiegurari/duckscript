use super::*;
use crate::test;
use crate::test::{ArrayCommand, CommandValidation};

#[test]
fn common_functions() {
    test::test_common_command_functions(create(""));
}

#[test]
fn run_no_args() {
    test::run_script_and_validate(
        vec![create(""), Box::new(ArrayCommand {})],
        r#"
        test_var = test_array 1 2 3
        out = dump_state
        "#,
        CommandValidation::Contains("out".to_string(), "3".to_string()),
    );
}
