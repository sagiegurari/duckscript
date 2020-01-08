use super::*;
use crate::test;
use crate::test::CommandValidation;

#[test]
fn common_functions() {
    test::test_common_command_functions(create(""));
}

#[test]
fn run_no_file_provided() {
    test::run_script_and_error(vec![create("")], "out = cat ./Cargo2.toml", "out");
}

#[test]
fn run_valid() {
    test::run_script_and_validate(
        vec![create("")],
        "out = cat ./Cargo.toml",
        CommandValidation::Contains("out".to_string(), "duckscript".to_string()),
    );
}
