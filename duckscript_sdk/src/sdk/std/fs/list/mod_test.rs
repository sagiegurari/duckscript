use super::*;
use crate::test;
use crate::test::CommandValidation;

#[test]
fn common_functions() {
    test::test_common_command_functions(create(""));
}

#[test]
fn run_no_args() {
    test::run_script_and_validate(
        vec![create("")],
        "out = ls",
        CommandValidation::Match("out".to_string(), "true".to_string()),
    );
}

#[test]
fn run_flag_only() {
    test::run_script_and_validate(
        vec![create("")],
        "out = ls -l",
        CommandValidation::Match("out".to_string(), "true".to_string()),
    );
}

#[test]
fn run_current_directory() {
    test::run_script_and_validate(
        vec![create("")],
        "out = ls .",
        CommandValidation::Match("out".to_string(), "true".to_string()),
    );
}

#[test]
fn run_current_directory_and_flags() {
    test::run_script_and_validate(
        vec![create("")],
        "out = ls -l .",
        CommandValidation::Match("out".to_string(), "true".to_string()),
    );
}

#[test]
fn run_current_directory_does_not_exist() {
    test::run_script_and_validate(
        vec![create("")],
        "out = ls ./baddirname",
        CommandValidation::Match("out".to_string(), "false".to_string()),
    );
}

#[test]
fn run_file() {
    test::run_script_and_validate(
        vec![create("")],
        "out = ls ./Cargo.toml",
        CommandValidation::Match("out".to_string(), "true".to_string()),
    );
}

#[test]
fn run_file_with_flags() {
    test::run_script_and_validate(
        vec![create("")],
        "out = ls -l ./Cargo.toml",
        CommandValidation::Match("out".to_string(), "true".to_string()),
    );
}
