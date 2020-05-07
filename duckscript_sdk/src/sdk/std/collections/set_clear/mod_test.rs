use super::*;
use crate::sdk::std::collections::{set, set_put, set_size};
use crate::test;
use crate::test::CommandValidation;

#[test]
fn common_functions() {
    test::test_common_command_functions(create(""));
}

#[test]
fn run_no_args() {
    test::run_script_and_error(vec![create("")], "out = set_clear", "out");
}

#[test]
fn run_not_found() {
    test::run_script_and_error(vec![create("")], "out = set_clear bad_handle", "out");
}

#[test]
fn run_found() {
    test::run_script_and_validate(
        vec![
            create(""),
            set::create(""),
            set_size::create(""),
            set_put::create(""),
        ],
        r#"
        handle = set_new
        set_put ${handle} 1
        set_put ${handle} 2
        set_put ${handle} 3
        set_put ${handle} 1
        set_clear ${handle}
        out = set_size ${handle}
        "#,
        CommandValidation::Match("out".to_string(), "0".to_string()),
    );
}
