use super::*;
use crate::test;
use crate::test::CommandValidation;

#[test]
fn common_functions() {
    test::test_common_command_functions(create(""));
}

#[cfg(windows)]
#[test]
fn run_windows() {
    test::run_script_and_error(vec![create("")], "out = os_version", "out");
}

#[test]
#[cfg(not(windows))]
fn run_valid() {
    test::run_script_and_validate(
        vec![create("")],
        "out = os_version",
        CommandValidation::Ignore,
    );
}
