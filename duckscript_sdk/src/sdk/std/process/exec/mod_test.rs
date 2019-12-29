use super::*;
use crate::test;
use crate::test::CommandValidation;

#[test]
fn common_functions() {
    let commands = create("");
    for command in commands {
        test::test_common_command_functions(command);
    }
}

#[test]
#[test]
fn run_no_args() {
    test::run_script_and_fail(create(""), "exec");
}

#[test]
fn run_no_output() {
    test::run_script_and_validate(create(""), "exec echo test", CommandValidation::None);
}

#[test]
fn run_with_output() {
    let context = test::run_script_and_validate(
        create(""),
        r#"
        out = exec echo 1 2 3
        stdout = get_stdout ${out}
        stderr = get_stderr ${out}
        exit_code = get_exit_code ${out}
        "#,
        CommandValidation::Contains("out".to_string(), "handle:".to_string()),
    );

    let stdout = context.variables.get("stdout").unwrap();
    let stderr = context.variables.get("stderr").unwrap();
    let exit_code = context.variables.get("exit_code").unwrap();

    assert!(stdout.contains("1 2 3"));
    assert!(stderr.is_empty());
    assert_eq!(exit_code, "0");
}

#[test]
fn run_error_code_with_output() {
    test::run_script_and_fail(create(""), "out = exec badcommand");
}

#[test]
fn run_error_code_no_output() {
    test::run_script_and_fail(create(""), "exec badcommand");
}
