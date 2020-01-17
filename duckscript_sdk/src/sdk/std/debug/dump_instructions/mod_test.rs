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
        "out = dump_instructions",
        CommandValidation::Contains("out".to_string(), "dump_instructions".to_string()),
    );
}
