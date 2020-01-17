use super::*;
use crate::test;
use crate::test::{CommandValidation, SetCommand};

#[test]
fn common_functions() {
    test::test_common_command_functions(create(""));
}

#[test]
fn run_no_args() {
    test::run_script_and_error(vec![create("")], "out = write_properties", "out");
}

#[test]
fn run_valid() {
    test::run_script_and_validate(
        vec![create(""), Box::new(SetCommand {})],
        r#"
        a = test_set 1
        b = test_set 2
        out = write_properties a b
        "#,
        CommandValidation::Contains("out".to_string(), "b=2".to_string()),
    );
}
