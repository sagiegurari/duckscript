use super::*;
use crate::test;
use crate::test::CommandValidation;

#[test]
fn common_functions() {
    test::test_common_command_functions(create(""));
}

#[test]
fn run_invalid_args() {
    test::run_script_and_crash(vec![create("")], "out = on_error");
}

#[test]
fn run_valid() {
    let context = test::run_script_and_validate(
        vec![create("")],
        "out = on_error 1 2 3",
        CommandValidation::Match("out".to_string(), "false".to_string()),
    );

    let sub_state_value = context.state.get("duckscriptsdk::on_error").unwrap();
    match sub_state_value {
        StateValue::SubState(sub_state) => {
            match sub_state.get("error").unwrap() {
                StateValue::String(value) => assert_eq!(value, "1"),
                _ => panic!("Invalid value type."),
            };
            match sub_state.get("line").unwrap() {
                StateValue::String(value) => assert_eq!(value, "2"),
                _ => panic!("Invalid value type."),
            };
            match sub_state.get("source").unwrap() {
                StateValue::String(value) => assert_eq!(value, "3"),
                _ => panic!("Invalid value type."),
            };
        }
        _ => panic!("Invalid sub state type."),
    }
}
