use super::*;
use crate::test;
use crate::test::CommandValidation;

#[test]
fn common_functions() {
    test::test_common_command_functions(create(""));
}

#[test]
fn run_no_args() {
    test::run_script_and_validate(vec![create("")], "out = trim", CommandValidation::None);
}

#[test]
fn run_with_spaces() {
    test::run_script_and_validate(
        vec![create("")],
        r#"out = trim "    some  text   " "#,
        CommandValidation::Match("out".to_string(), "some  text".to_string()),
    );
}
