use super::*;
use crate::test;
use crate::test::CommandValidation;

#[test]
fn common_functions() {
    test::test_common_command_functions(create(""));
}

#[test]
fn run_no_arguments() {
    test::run_script_and_error(vec![create("")], "out = which", "out");
}

#[test]
fn run_not_found() {
    test::run_script_and_validate(
        vec![create("")],
        "out = which bad_executable",
        CommandValidation::None,
    );
}

#[test]
fn run_found() {
    test::run_script_and_validate(
        vec![create("")],
        "out = which rustc",
        CommandValidation::Ignore,
    );
}
