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
    let mut context = test::run_script_and_validate(
        vec![create("")],
        "out = array",
        CommandValidation::Contains("out".to_string(), "handle:".to_string()),
    );

    let state = get_handles_sub_state(&mut context.state);
    let list_value = state.get(context.variables.get("out").unwrap()).unwrap();
    match list_value {
        StateValue::List(list) => assert!(list.is_empty()),
        _ => panic!("Invalid handle type."),
    }
}

#[test]
fn run_with_args() {
    let mut context = test::run_script_and_validate(
        vec![create("")],
        r#"out = array 1 "hello world" test"#,
        CommandValidation::Contains("out".to_string(), "handle:".to_string()),
    );

    let state = get_handles_sub_state(&mut context.state);
    let list_value = state.remove(context.variables.get("out").unwrap()).unwrap();
    match list_value {
        StateValue::List(mut list) => {
            assert_eq!(list.len(), 3);

            match list.pop().unwrap() {
                StateValue::String(value) => assert_eq!(value, "test"),
                _ => panic!("Invalid handle value."),
            };

            match list.pop().unwrap() {
                StateValue::String(value) => assert_eq!(value, "hello world"),
                _ => panic!("Invalid handle value."),
            };

            match list.pop().unwrap() {
                StateValue::String(value) => assert_eq!(value, "1"),
                _ => panic!("Invalid handle value."),
            };
        }
        _ => panic!("Invalid handle type."),
    }
}
