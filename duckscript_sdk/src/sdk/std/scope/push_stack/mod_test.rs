use super::*;
use crate::test;
use crate::test::{CommandValidation, SetCommand};

#[test]
fn common_functions() {
    test::test_common_command_functions(create(""));
}

#[test]
fn run_no_args() {
    let context = test::run_script_and_validate(
        vec![create(""), Box::new(SetCommand {})],
        r#"
        a = test_set 1
        b = test_set 2
        scope_push_stack
        "#,
        CommandValidation::None,
    );

    let variables = context.variables;
    assert!(variables.is_empty());
}

#[test]
fn run_keep_variables() {
    let context = test::run_script_and_validate(
        vec![create(""), Box::new(SetCommand {})],
        r#"
        a = test_set 1
        b = test_set 2
        c = test_set 3
        scope_push_stack --copy b c
        "#,
        CommandValidation::Ignore,
    );

    let variables = context.variables;
    assert_eq!(variables.len(), 2);
    assert_eq!(variables.get("b").unwrap(), "2");
    assert_eq!(variables.get("c").unwrap(), "3");
}
