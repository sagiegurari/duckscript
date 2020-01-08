use super::*;
use crate::test;
use crate::test::CommandValidation;

#[test]
fn common_functions() {
    test::test_common_command_functions(create(""));
}

#[test]
fn run_valid() {
    test::run_script_and_validate(
        vec![create("")],
        "out = os_family",
        CommandValidation::Any(
            "out".to_string(),
            vec![
                "windows".to_string(),
                "linux".to_string(),
                "mac".to_string(),
            ],
        ),
    );
}
