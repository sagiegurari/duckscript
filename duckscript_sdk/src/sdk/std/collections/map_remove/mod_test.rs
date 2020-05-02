use super::*;
use crate::sdk::std::collections::{map, map_put};
use crate::sdk::std::var::set;
use crate::test;
use crate::test::CommandValidation;

#[test]
fn common_functions() {
    test::test_common_command_functions(create(""));
}

#[test]
fn run_no_args() {
    test::run_script_and_error(vec![create("")], "out = map_remove", "out");
}

#[test]
fn run_missing_key() {
    test::run_script_and_error(vec![create("")], "out = map_remove handle", "out");
}

#[test]
fn run_not_found() {
    test::run_script_and_error(vec![create("")], "out = map_remove bad_handle key", "out");
}

#[test]
fn run_found() {
    test::run_script_and_validate(
        vec![create(""), map::create(""), map_put::create("")],
        r#"
        handle = map
        map_put ${handle} key value
        out = map_remove ${handle} key
        "#,
        CommandValidation::Match("out".to_string(), "value".to_string()),
    );
}

#[test]
fn run_twice() {
    test::run_script_and_validate(
        vec![
            create(""),
            map::create(""),
            map_put::create(""),
            set::create(""),
        ],
        r#"
        handle = map
        map_put ${handle} key value
        out = map_remove ${handle} key
        out = map_remove ${handle} key
        handle = set
        "#,
        CommandValidation::None,
    );
}
