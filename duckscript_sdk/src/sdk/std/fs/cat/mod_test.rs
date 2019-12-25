use super::*;
use crate::test;

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
        Some("out".to_string()),
        Some("duckscript".to_string()),
        true,
    );
}
