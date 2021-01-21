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
    test::run_script_and_error(vec![create("")], "out = exec", "out");
}

#[test]
fn run_no_output() {
    let script = format!("exec {} \"echo test\"", COMMAND_RUNNER);
    test::run_script_and_validate(vec![create("")], &script, CommandValidation::None);
}

#[test]
fn run_no_output_with_fail_on_error_valid() {
    let script = format!("exec --fail-on-error {} \"echo test\"", COMMAND_RUNNER);

    test::run_script_and_validate(
        vec![create("")],
        &script,
        CommandValidation::None,
    );
}

#[test]
fn run_no_output_with_fail_on_error_invalid() {
    let script = format!("exec --fail-on-error {} badcommand", COMMAND_RUNNER);

    test::run_script_and_error(vec![create("")], &script, "");
}

#[test]
fn run_with_output() {
    let script = format!("out = exec --fail-on-error {} \"echo 1 2 3\"", COMMAND_RUNNER);

    let context = test::run_script_and_validate(
        vec![create("")],
        &script,
        CommandValidation::Match("out.code".to_string(), "0".to_string()),
    );

    let stdout = context.variables.get("out.stdout").unwrap();
    let stderr = context.variables.get("out.stderr").unwrap();
    let exit_code = context.variables.get("out.code").unwrap();

    assert!(stdout.contains("1 2 3"));
    assert!(stderr.is_empty());
    assert_eq!(exit_code, "0");
}

#[test]
fn run_error_code_with_output() {
    test::run_script_and_error(vec![create("")], "out = exec badcommand", "out");
}

#[test]
fn run_get_exit_code() {
    let script = format!("out = exec --get-exit-code {} \"exit 42\"", COMMAND_RUNNER);

    test::run_script_and_validate(
        vec![create("")],
        &script,
        CommandValidation::Match("out".to_string(), "42".to_string()),
    );
}