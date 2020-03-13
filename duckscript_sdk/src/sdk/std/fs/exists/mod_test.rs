use super::*;
use crate::test;
use crate::test::CommandValidation;

#[test]
fn common_functions() {
    test::test_common_command_functions(create(""));
}

#[test]
fn run_no_path_provided() {
    test::run_script_and_error(vec![create("")], "out = is_path_exists", "out");
}

#[test]
fn run_file() {
    test::run_script_and_validate(
        vec![create("")],
        "out = is_path_exists ./Cargo.toml",
        CommandValidation::Match("out".to_string(), "true".to_string()),
    );
}

#[test]
fn run_directory() {
    test::run_script_and_validate(
        vec![create("")],
        "out = is_path_exists ./src",
        CommandValidation::Match("out".to_string(), "true".to_string()),
    );
}

#[test]
fn run_not_found() {
    test::run_script_and_validate(
        vec![create("")],
        "out = is_path_exists ./badpath",
        CommandValidation::Match("out".to_string(), "false".to_string()),
    );
}
