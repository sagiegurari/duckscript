use super::*;
use crate::sdk::std::var::set;
use crate::test;
use crate::test::CommandValidation;

#[test]
fn common_functions() {
    test::test_common_command_functions(create(""));
}

#[test]
fn run_no_arguments() {
    test::run_script_and_validate(
        vec![create("")],
        "out = get_by_name",
        CommandValidation::None,
    );
}

#[test]
fn run_variable_not_found() {
    test::run_script_and_validate(
        vec![create("")],
        "out = get_by_name test",
        CommandValidation::None,
    );
}

#[test]
fn run_variable_found() {
    test::run_script_and_validate(
        vec![create(""), set::create("")],
        r#"
        test = set value
        out = get_by_name test
        "#,
        CommandValidation::Match("out".to_string(), "value".to_string()),
    );
}
