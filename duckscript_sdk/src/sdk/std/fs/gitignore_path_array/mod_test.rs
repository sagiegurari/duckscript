use super::*;
use crate::test;

#[test]
fn common_functions() {
    test::test_common_command_functions(create(""));
}

#[test]
fn run_no_pattern_provided() {
    test::run_script_and_error(vec![create("")], "out = gitignore_path_array", "out");
}
