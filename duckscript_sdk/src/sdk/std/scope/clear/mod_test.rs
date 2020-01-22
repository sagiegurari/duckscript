use super::*;
use crate::test;
use crate::test::{CommandValidation, SetCommand};

#[test]
fn common_functions() {
    test::test_common_command_functions(create(""));
}

#[test]
fn run_no_args() {
    test::run_script_and_error(vec![create("")], "out = clear_scope", "out");
}

#[test]
fn run_not_found() {
    test::run_script_and_validate(
        vec![create("")],
        "out = clear_scope myscope",
        CommandValidation::None,
    );
}

#[test]
fn run_found() {
    test::run_script_and_validate(
        vec![create(""), Box::new(SetCommand {})],
        r#"
        myscope::1 = test_set 1
        myscope::2 = test_set 1
        out = clear_scope myscope
        "#,
        CommandValidation::None,
    );
}
