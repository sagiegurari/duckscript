use super::*;
use crate::sdk::std::collections::{array_pop, map, map_put};
use crate::test;
use crate::test::CommandValidation;

#[test]
fn common_functions() {
    test::test_common_command_functions(create(""));
}

#[test]
fn run_no_args() {
    test::run_script_and_error(vec![create("")], "out = map_keys", "out");
}

#[test]
fn run_not_found() {
    test::run_script_and_error(vec![create("")], "out = map_keys bad_handle", "out");
}

#[test]
fn run_valid() {
    test::run_script_and_validate(
        vec![
            create(""),
            map::create(""),
            map_put::create(""),
            array_pop::create(""),
        ],
        r#"
        handle = map
        map_put ${handle} a 1
        keys = map_keys ${handle}
        out = array_pop ${keys}
        "#,
        CommandValidation::Match("out".to_string(), "a".to_string()),
    );
}
