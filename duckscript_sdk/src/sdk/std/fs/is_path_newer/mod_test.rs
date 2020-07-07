use super::*;
use crate::test;

#[test]
fn common_functions() {
    test::test_common_command_functions(create(""));
}

#[test]
fn run_no_path_provided() {
    test::run_script_and_error(vec![create("")], "out = is_path_newer", "out");
}

#[test]
fn run_not_found() {
    test::run_script_and_error(
        vec![create("")],
        "out = is_path_newer ./badpath ./Cargo.toml",
        "out",
    );
}
