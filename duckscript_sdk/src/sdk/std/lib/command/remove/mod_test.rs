use super::*;
use crate::test;
use crate::test::{CommandValidation, EmptyCommand};

#[test]
fn common_functions() {
    test::test_common_command_functions(create(""));
}

#[test]
fn run_no_args() {
    test::run_script_and_error(vec![create("")], "out = remove_command", "out");
}

#[test]
fn run_valid() {
    let context = test::run_script_and_validate(
        vec![create(""), Box::new(EmptyCommand {})],
        r#"
    out = remove_command test_empty
    "#,
        CommandValidation::Match("out".to_string(), "true".to_string()),
    );

    assert!(!context.commands.exists("test_empty"));
    assert!(!context.commands.exists("test_empty1"));
    assert!(!context.commands.exists("test_empty2"));
    assert!(context.commands.exists("remove_command"));
}
