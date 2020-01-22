use super::*;
use crate::sdk::std::collections::{array, array_length};
use crate::sdk::std::scope::clear;
use crate::sdk::std::string::equals;
use crate::test;
use crate::test::CommandValidation;

#[test]
fn common_functions() {
    test::test_common_command_functions(create(""));
}

#[test]
fn run_not_empty() {
    test::run_script_and_validate(
        vec![
            create(""),
            clear::create(""),
            array::create(""),
            array_length::create(""),
            equals::create(""),
        ],
        r#"
        values = array a b c
        out = array_is_empty ${values}
        "#,
        CommandValidation::Match("out".to_string(), "false".to_string()),
    );
}

#[test]
fn run_empty() {
    test::run_script_and_validate(
        vec![
            create(""),
            clear::create(""),
            array::create(""),
            array_length::create(""),
            equals::create(""),
        ],
        r#"
        values = array
        out = array_is_empty ${values}
        "#,
        CommandValidation::Match("out".to_string(), "true".to_string()),
    );
}
