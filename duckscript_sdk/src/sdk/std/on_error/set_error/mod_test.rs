use super::*;
use crate::test;
use crate::test::CommandValidation;

#[test]
fn common_functions() {
    test::test_common_command_functions(create(""));
}

#[test]
fn run_invalid_args() {
    test::run_script_and_error(vec![create("")], "out = set_error", "out");
}

#[test]
fn run_valid() {
    let context = test::run_script_and_validate(
        vec![create("")],
        "out = set_error text",
        CommandValidation::None,
    );

    let sub_state_value = context.state.get("duckscriptsdk::on_error").unwrap();
    match sub_state_value {
        StateValue::SubState(sub_state) => {
            match sub_state.get("error").unwrap() {
                StateValue::String(value) => assert_eq!(value, "text"),
                _ => panic!("Invalid value type."),
            };
            match sub_state.get("line").unwrap() {
                StateValue::String(value) => assert_eq!(value, "0"),
                _ => panic!("Invalid value type."),
            };
            assert!(sub_state.get("source").is_none());
        }
        _ => panic!("Invalid sub state type."),
    }
}
