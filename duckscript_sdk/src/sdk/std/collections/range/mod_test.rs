use super::*;
use crate::test;
use crate::test::CommandValidation;
use crate::utils::state::get_handles_sub_state;

#[test]
fn common_functions() {
    test::test_common_command_functions(create(""));
}

#[test]
fn run_no_args() {
    test::run_script_and_fail(vec![create("")], "out = range");
}

#[test]
fn run_single_arg() {
    test::run_script_and_fail(vec![create("")], "out = range 1");
}

#[test]
fn run_non_numeric_start() {
    test::run_script_and_fail(vec![create("")], "out = range a 10");
}

#[test]
fn run_non_numeric_end() {
    test::run_script_and_fail(vec![create("")], "out = range 1 b");
}

#[test]
fn run_decrease() {
    test::run_script_and_fail(vec![create("")], "out = range 2 1");
}

#[test]
fn runsame_value() {
    let mut context = test::run_script_and_validate(
        vec![create("")],
        "out = range 5 5",
        CommandValidation::Contains("out".to_string(), "handle:".to_string()),
    );

    let state = get_handles_sub_state(&mut context.state);
    let list_value = state.remove(context.variables.get("out").unwrap()).unwrap();
    match list_value {
        StateValue::List(list) => assert_eq!(list.len(), 0),
        _ => panic!("Invalid handle type."),
    }
}

#[test]
fn run_positive() {
    let mut context = test::run_script_and_validate(
        vec![create("")],
        "out = range 5 10",
        CommandValidation::Contains("out".to_string(), "handle:".to_string()),
    );

    let state = get_handles_sub_state(&mut context.state);
    let list_value = state.remove(context.variables.get("out").unwrap()).unwrap();
    match list_value {
        StateValue::List(mut list) => {
            assert_eq!(list.len(), 5);

            for index in 5..10 {
                match list.remove(0) {
                    StateValue::Number32Bit(value) => assert_eq!(value, index),
                    _ => panic!("Invalid handle value."),
                };
            }
        }
        _ => panic!("Invalid handle type."),
    }
}

#[test]
fn run_negative() {
    let mut context = test::run_script_and_validate(
        vec![create("")],
        "out = range -5 10",
        CommandValidation::Contains("out".to_string(), "handle:".to_string()),
    );

    let state = get_handles_sub_state(&mut context.state);
    let list_value = state.remove(context.variables.get("out").unwrap()).unwrap();
    match list_value {
        StateValue::List(mut list) => {
            assert_eq!(list.len(), 15);

            for index in -5..10 {
                match list.remove(0) {
                    StateValue::Number32Bit(value) => assert_eq!(value, index),
                    _ => panic!("Invalid handle value."),
                };
            }
        }
        _ => panic!("Invalid handle type."),
    }
}
