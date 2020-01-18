use super::*;
use crate::test;
use crate::test::CommandValidation;

#[test]
fn common_functions() {
    test::test_common_command_functions(create(""));
}

#[test]
fn run_no_args() {
    test::run_script_and_error(vec![create("")], "out = replace", "out");
}

#[test]
fn run_single_argument() {
    test::run_script_and_error(vec![create("")], "out = replace true", "out");
}

#[test]
fn run_two_arguments() {
    test::run_script_and_error(vec![create("")], "out = replace true true", "out");
}

#[test]
fn run_found() {
    test::run_script_and_validate(
        vec![create("")],
        "out = replace bigtextbigtext ig aa",
        CommandValidation::Match("out".to_string(), "baatextbaatext".to_string()),
    );
}

#[test]
fn run_not_found() {
    test::run_script_and_validate(
        vec![create("")],
        "out = replace text ig aa",
        CommandValidation::Match("out".to_string(), "text".to_string()),
    );
}
