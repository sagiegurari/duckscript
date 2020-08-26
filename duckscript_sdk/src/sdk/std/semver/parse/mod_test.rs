use super::*;
use crate::test;
use crate::test::CommandValidation;

#[test]
fn common_functions() {
    test::test_common_command_functions(create(""));
}

#[test]
fn run_no_args() {
    test::run_script_and_error(vec![create("")], "out = semver_parse", "out");
}

#[test]
fn run_invalid() {
    test::run_script_and_error(vec![create("")], "out = semver_parse aaa_bbb", "out");
}

#[test]
fn run_major() {
    test::run_script_and_validate(
        vec![create("")],
        "out = semver_parse 1.2.3",
        CommandValidation::Match("out.major".to_string(), "1".to_string()),
    );
}

#[test]
fn run_minor() {
    test::run_script_and_validate(
        vec![create("")],
        "out = semver_parse 1.2.3",
        CommandValidation::Match("out.minor".to_string(), "2".to_string()),
    );
}

#[test]
fn run_patch() {
    test::run_script_and_validate(
        vec![create("")],
        "out = semver_parse 1.2.3",
        CommandValidation::Match("out.patch".to_string(), "3".to_string()),
    );
}
