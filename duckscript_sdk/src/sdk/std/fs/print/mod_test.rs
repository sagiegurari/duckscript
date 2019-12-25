use super::*;
use crate::test;
use crate::test::CommandValidation;

#[test]
fn common_functions() {
    test::test_common_command_functions(create(""));
}

#[test]
fn run_no_file_provided() {
    test::run_command_and_fail(create(""), "cat ./Cargo2.toml");
}

#[test]
fn run_valid() {
    test::validate_command(
        create(""),
        "out = cat ./Cargo.toml",
        CommandValidation::Contains("out".to_string(), "duckscript".to_string()),
    );
}
