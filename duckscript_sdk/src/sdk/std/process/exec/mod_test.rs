use super::*;
use crate::test;
use crate::test::CommandValidation;

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
    test::run_script_and_validate(vec![create("")], "exec echo test", CommandValidation::None);
}

#[test]
fn run_with_output() {
    let context = test::run_script_and_validate(
        vec![create("")],
        "out = exec echo 1 2 3",
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
