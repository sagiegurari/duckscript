use super::*;
use crate::sdk::std::collections::{map, map_put, map_size};
use crate::test;
use crate::test::CommandValidation;

#[test]
fn common_functions() {
    test::test_common_command_functions(create(""));
}

#[test]
fn run_no_args() {
    test::run_script_and_error(vec![create("")], "out = map_clear", "out");
}

#[test]
fn run_not_found() {
    test::run_script_and_error(vec![create("")], "out = map_clear bad_handle", "out");
}

#[test]
fn run_found() {
    test::run_script_and_validate(
        vec![
            create(""),
            map::create(""),
            map_size::create(""),
            map_put::create(""),
        ],
        r#"
        handle = map
        map_put ${handle} a 1
        map_put ${handle} b 2
        map_put ${handle} c 3
        map_put ${handle} a 4
        map_clear ${handle}
        out = map_size ${handle}
        "#,
        CommandValidation::Match("out".to_string(), "0".to_string()),
    );
}
