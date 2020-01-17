use super::*;
use crate::test;
use crate::test::{CommandValidation, SetCommand};

#[test]
fn common_functions() {
    test::test_common_command_functions(create(""));
}

#[test]
fn run_no_args() {
    test::run_script_and_validate(
        vec![create(""), Box::new(SetCommand {})],
        r#"
        test_var = test_set 1
        out = dump_variables
        "#,
        CommandValidation::Contains("out".to_string(), "test_var".to_string()),
    );
}
