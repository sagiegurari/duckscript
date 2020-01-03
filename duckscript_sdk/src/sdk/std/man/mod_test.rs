use super::*;
use crate::sdk::std::alias;
use crate::test;
use crate::test::{CommandValidation, SetCommand};

#[test]
fn common_functions() {
    test::test_common_command_functions(create(""));
}

#[test]
fn run_no_args() {
    test::run_script_and_validate(
        vec![create("")],
        "out = man",
        CommandValidation::Contains("out".to_string(), " = man".to_string()),
    );
}

#[test]
fn run_self_command_found() {
    test::run_script_and_validate(
        vec![create("")],
        "out = man man",
        CommandValidation::Contains("out".to_string(), " = man".to_string()),
    );
}

#[test]
fn run_command_not_found() {
    test::run_script_and_fail(vec![create("")], "out = man badcommand");
}

#[test]
fn run_command_found_with_docs() {
    test::run_script_and_validate(
        vec![create(""), alias::create("")],
        "out = man alias",
        CommandValidation::Contains("out".to_string(), "alias ".to_string()),
    );
}

#[test]
fn run_command_found_no_docs() {
    test::run_script_and_validate(
        vec![create(""), Box::new(SetCommand {})],
        "out = man test_set",
        CommandValidation::None,
    );
}
