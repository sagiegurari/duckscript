use super::*;
use crate::test;

#[test]
fn common_functions() {
    test::test_common_command_functions(create(""));
}

#[test]
fn run_no_args() {
    test::run_script_and_error(vec![create("")], "out = semver_is_newer", "out");
}

#[test]
fn run_single_arg() {
    test::run_script_and_error(vec![create("")], "out = semver_is_newer 1.2.3", "out");
}

#[test]
fn run_invalid_args() {
    test::run_script_and_error(
        vec![create("")],
        "out = semver_is_newer abc_123 123_test",
        "out",
    );
}
