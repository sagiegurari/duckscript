use super::*;
use crate::test;
use crate::test::CommandValidation;
use crate::utils::state::get_handles_sub_state;
use std::str;

#[test]
fn common_functions() {
    test::test_common_command_functions(create(""));
}

#[test]
fn run_no_args() {
    test::run_script_and_error(vec![create("")], "out = string_to_bytes", "out");
}

#[test]
fn run_valid() {
    let mut context = test::run_script_and_validate(
        vec![create("")],
        r#"out = string_to_bytes "hello world""#,
        CommandValidation::Ignore,
    );

    let state = get_handles_sub_state(&mut context.state);
    let list_value = state.remove(context.variables.get("out").unwrap()).unwrap();
    match list_value {
        StateValue::ByteArray(binary) => {
            let text = str::from_utf8(&binary).unwrap();
            assert_eq!(text, "hello world");
        }
        _ => panic!("Invalid type."),
    }
}
