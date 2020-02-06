use super::*;
use crate::sdk::std::collections::{map, map_get};
use crate::test;
use crate::test::{CommandValidation, SetCommand};

#[test]
fn common_functions() {
    test::test_common_command_functions(create(""));
}

#[test]
fn run_no_args() {
    test::run_script_and_error(vec![create("")], "out = map_load_properties", "out");
}

#[test]
fn run_missing_text() {
    test::run_script_and_error(vec![create("")], "out = map_load_properties handle", "out");
}

#[test]
fn run_not_found() {
    test::run_script_and_error(
        vec![create("")],
        "out = map_load_properties bad_handle key=value",
        "out",
    );
}

#[test]
fn run_valid() {
    test::run_script_and_validate(
        vec![
            create(""),
            map::create(""),
            map_get::create(""),
            Box::new(SetCommand {}),
        ],
        r#"
        props = test_set "a=1\nb=2"
        handle = map
        map_load_properties ${handle} ${props}
        out = map_get ${handle} b
        "#,
        CommandValidation::Match("out".to_string(), "2".to_string()),
    );
}

#[test]
fn run_with_prefix() {
    test::run_script_and_validate(
        vec![
            create(""),
            map::create(""),
            map_get::create(""),
            Box::new(SetCommand {}),
        ],
        r#"
        props = test_set "a=1\nb=2"
        handle = map
        map_load_properties --prefix config ${handle} ${props}
        out = map_get ${handle} config.b
        "#,
        CommandValidation::Match("out".to_string(), "2".to_string()),
    );
}
