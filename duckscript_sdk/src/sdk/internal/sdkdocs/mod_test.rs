use super::*;
use crate::test;

#[test]
fn common_functions() {
    test::test_common_command_functions(create(""));
}

#[test]
fn run_no_file_provided() {
    test::run_command_and_fail(create("internal"), "internal::sdkdocs");
}

#[test]
fn run_valid() {
    test::validate_command(
        create("internal"),
        "out = internal::sdkdocs ./target/temp.md",
        Some("out".to_string()),
        Some("./target/temp.md".to_string()),
        false,
    );
}
