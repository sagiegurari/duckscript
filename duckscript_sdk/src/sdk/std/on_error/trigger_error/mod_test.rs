use super::*;
use crate::test;

#[test]
fn common_functions() {
    test::test_common_command_functions(create(""));
}

#[test]
fn run_no_args() {
    test::run_script_and_error(vec![create("")], "out = trigger_error", "out");
}

#[test]
fn run_with_message() {
    test::run_script_and_error(vec![create("")], "out = trigger_error msg", "out");
}
