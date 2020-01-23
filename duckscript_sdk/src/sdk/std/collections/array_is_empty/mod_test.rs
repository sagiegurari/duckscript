use super::*;
use crate::sdk::std::collections::{array, array_length};
use crate::sdk::std::release;
use crate::sdk::std::scope::clear;
use crate::sdk::std::string::equals;
use crate::test;
use crate::test::CommandValidation;

#[test]
fn common_functions() {
    test::test_common_command_functions(create("").unwrap());
}

#[test]
fn run_not_empty() {
    let context = test::run_script_and_validate(
        vec![
            create("").unwrap(),
            clear::create(""),
            release::create(""),
            array::create(""),
            array_length::create(""),
            equals::create(""),
        ],
        r#"
        values = array a b c
        out = array_is_empty ${values}
        release ${values}
        "#,
        CommandValidation::Match("out".to_string(), "false".to_string()),
    );

    assert_eq!(context.variables.len(), 2);
    test::is_handles_empty(&context.state);
}

#[test]
fn run_empty() {
    let context = test::run_script_and_validate(
        vec![
            create("").unwrap(),
            clear::create(""),
            release::create(""),
            array::create(""),
            array_length::create(""),
            equals::create(""),
        ],
        r#"
        values = array
        out = array_is_empty ${values}
        release ${values}
        "#,
        CommandValidation::Match("out".to_string(), "true".to_string()),
    );

    assert_eq!(context.variables.len(), 2);
    test::is_handles_empty(&context.state);
}
