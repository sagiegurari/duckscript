use super::*;
use crate::test;
use crate::test::CommandValidation;

#[test]
fn common_functions() {
    test::test_common_command_functions(create(""));
}

#[test]
fn run_no_args() {
    test::run_script_and_error(vec![create("")], "out = substring", "out");
}

#[test]
fn run_empty_text() {
    test::run_script_and_validate(
        vec![create("")],
        r#"out = substring """#,
        CommandValidation::Match("out".to_string(), "".to_string()),
    );
}

#[test]
fn run_only_text() {
    test::run_script_and_validate(
        vec![create("")],
        "out = substring text",
        CommandValidation::Match("out".to_string(), "text".to_string()),
    );
}

#[test]
fn run_text_only_start() {
    test::run_script_and_validate(
        vec![create("")],
        "out = substring text 1",
        CommandValidation::Match("out".to_string(), "ext".to_string()),
    );
}

#[test]
fn run_text_only_end() {
    test::run_script_and_validate(
        vec![create("")],
        "out = substring abcd -1",
        CommandValidation::Match("out".to_string(), "abc".to_string()),
    );
}

#[test]
fn run_text_start_and_end() {
    test::run_script_and_validate(
        vec![create("")],
        "out = substring text 1 3",
        CommandValidation::Match("out".to_string(), "ex".to_string()),
    );
}

#[test]
fn run_text_start_and_end_same() {
    test::run_script_and_validate(
        vec![create("")],
        "out = substring text 1 1",
        CommandValidation::Match("out".to_string(), "".to_string()),
    );
}

#[test]
fn run_text_start_bigger_than_end() {
    test::run_script_and_error(vec![create("")], "out = substring text 3 1", "out");
}

#[test]
fn run_text_only_start_bigger_than_text() {
    test::run_script_and_error(vec![create("")], "out = substring text 4", "out");
}

#[test]
fn run_text_only_end_bigger_than_text() {
    test::run_script_and_error(vec![create("")], "out = substring text -5", "out");
}

#[test]
fn run_text_start_and_end_end_bigger_than_text() {
    test::run_script_and_error(vec![create("")], "out = substring text 1 5", "out");
}

#[test]
fn run_text_start_and_end_both_bigger_than_text() {
    test::run_script_and_error(vec![create("")], "out = substring text 5 6", "out");
}
