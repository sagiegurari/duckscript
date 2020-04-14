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
        r#"out = env_to_map"#,
        CommandValidation::Contains("out".to_string(), "handle:".to_string()),
    );
}
