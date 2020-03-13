use super::*;
use crate::test;
use crate::test::CommandValidation;

#[test]
fn common_functions() {
    test::test_common_command_functions(create(""));
}

#[test]
fn run_no_path_provided() {
    test::run_script_and_error(vec![create("")], "out = is_readonly", "out");
}

#[test]
fn run_not_readonly() {
    test::run_script_and_validate(
        vec![create("")],
        "out = is_readonly ./Cargo.toml",
        CommandValidation::Match("out".to_string(), "false".to_string()),
    );
}
