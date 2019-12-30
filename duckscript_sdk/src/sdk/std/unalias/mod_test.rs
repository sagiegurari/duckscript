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
    test::run_script_and_fail(vec![create("")], "unalias");
}

#[test]
fn run_valid_no_alias_defined() {
    test::run_script_and_validate(
        vec![create(""), Box::new(SetCommand {})],
        r#"
    out = unalias test_set_alias
    "#,
        CommandValidation::Match("out".to_string(), "false".to_string()),
    );
}

#[test]
fn run_valid_alias_defined() {
    test::run_script_and_validate(
        vec![create(""), alias::create(""), Box::new(SetCommand {})],
        r#"
    alias set test_set
    test = set test
    out = unalias set
    "#,
        CommandValidation::Match("out".to_string(), "true".to_string()),
    );
}

#[test]
fn run_after_unalias() {
    test::run_script_and_fail(
        vec![create(""), alias::create(""), Box::new(SetCommand {})],
        r#"
    alias set test_set
    test = set test
    out = unalias set
    test = set test
    "#,
    );
}
