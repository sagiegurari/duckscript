use super::*;
use crate::test;
use crate::test::CommandValidation;

const COMMAND_RUNNER: &str = if cfg!(windows) { "cmd /c" } else { "sh -c" };

#[test]
fn common_functions() {
    test::test_common_command_functions(create(""));
}

#[test]
fn run_no_args() {
    test::run_script_and_error(vec![create("")], "out = spawn", "out");
}

#[test]
fn run_valid() {
    let script = format!("out = spawn {} echo test", COMMAND_RUNNER);

    test::run_script_and_validate(
        vec![create("")],
        &script,
        CommandValidation::PositiveNumber("out".to_string()),
    );
}

#[test]
fn run_valid_silent() {
    let script = format!("out = spawn --silent {} echo test", COMMAND_RUNNER);

    test::run_script_and_validate(
        vec![create("")],
        &script,
        CommandValidation::PositiveNumber("out".to_string()),
    );
}
