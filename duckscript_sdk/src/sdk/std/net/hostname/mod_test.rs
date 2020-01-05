use super::*;
use crate::test;
use crate::test::CommandValidation;

#[test]
fn common_functions() {
    test::test_common_command_functions(create(""));
}

#[test]
fn run_valid() {
    let name = hostname::get().unwrap();

    test::run_script_and_validate(
        vec![create("")],
        "out = hostname",
        CommandValidation::Match("out".to_string(), name.to_string_lossy().into_owned()),
    );
}
