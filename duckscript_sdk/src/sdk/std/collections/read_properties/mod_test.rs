use super::*;
use crate::test;
use crate::test::{CommandValidation, SetCommand};

#[test]
fn common_functions() {
    test::test_common_command_functions(create(""));
}

#[test]
fn run_no_args() {
    test::run_script_and_error(vec![create("")], "out = read_properties", "out");
}

#[test]
fn run_valid() {
    let context = test::run_script_and_validate(
        vec![create(""), Box::new(SetCommand {})],
        r#"
        props = test_set "a=1\nb=2"
        out = read_properties ${props}
        "#,
        CommandValidation::Match("out".to_string(), "2".to_string()),
    );

    assert_eq!(context.variables.get("a").unwrap(), "1");
    assert_eq!(context.variables.get("b").unwrap(), "2");
}

#[test]
fn run_with_prefix() {
    let context = test::run_script_and_validate(
        vec![create(""), Box::new(SetCommand {})],
        r#"
        props = test_set "a=1\nb=2"
        out = read_properties --prefix config ${props}
        "#,
        CommandValidation::Match("out".to_string(), "2".to_string()),
    );

    assert_eq!(context.variables.get("config.a").unwrap(), "1");
    assert_eq!(context.variables.get("config.b").unwrap(), "2");
}
