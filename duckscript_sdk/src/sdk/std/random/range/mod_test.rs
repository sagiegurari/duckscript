use super::*;
use crate::test;
use crate::test::CommandValidation;

#[test]
fn common_functions() {
    test::test_common_command_functions(create(""));
}

#[test]
fn run_no_args() {
    test::run_script_and_error(vec![create("")], "out = rand_range", "out");
}

#[test]
fn run_single_arg() {
    test::run_script_and_error(vec![create("")], "out = rand_range 1", "out");
}

#[test]
fn run_min_bigger_than_max() {
    test::run_script_and_error(vec![create("")], "out = rand_range 10 5", "out");
}

#[test]
fn run_valid() {
    test::run_script_and_validate(
        vec![create("")],
        "out = rand_range 10 15",
        CommandValidation::PositiveNumber("out".to_string()),
    );
}
