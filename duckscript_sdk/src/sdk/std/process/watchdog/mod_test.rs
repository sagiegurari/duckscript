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
    test::run_script_and_error(vec![create("")], "out = watchdog", "out");
}

#[test]
fn run_no_command() {
    test::run_script_and_error(
        vec![create("")],
        "out = watchdog --max-retries 0 --interval 0",
        "out",
    );
}

#[test]
fn run_without_retries() {
    let script = format!("out = watchdog --max-retries 0 --interval 0 -- {} echo 1 2 3", COMMAND_RUNNER);

    test::run_script_and_validate(
        vec![create("")],
        &script,
        CommandValidation::Match("out".to_string(), "1".to_string()),
    );
}

#[test]
fn run_with_retries() {
    let script = format!("out = watchdog --max-retries 3 --interval 0 -- {} echo 1 2 3", COMMAND_RUNNER);

    test::run_script_and_validate(
        vec![create("")],
        &script,
        CommandValidation::Match("out".to_string(), "4".to_string()),
    );
}

#[test]
fn run_error_code_with_output() {
    test::run_script_and_error(vec![create("")], "out = watchdog badcommand", "out");
}
