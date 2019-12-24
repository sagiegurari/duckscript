use super::*;
use crate::test;

#[test]
fn common_functions() {
    test::test_common_command_functions(create(""));
}

#[test]
fn run_no_file_provided() {
    let command = create("");
    test::run_command_and_fail_with_default_context(command, "readfile ./Cargo2.toml");
}

#[test]
fn run_valid() {
    let command = create("");
    let context =
        test::run_command_valid_with_default_context(command, "out = readfile ./Cargo.toml");

    assert!(context.variables.get("out").unwrap().contains("duckscript"));
}
