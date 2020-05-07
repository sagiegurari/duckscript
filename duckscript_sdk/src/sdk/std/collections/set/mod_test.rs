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
        "out = set_new",
        CommandValidation::Contains("out".to_string(), "handle:".to_string()),
    );

    let state = get_handles_sub_state(&mut context.state);
    let list_value = state.get(context.variables.get("out").unwrap()).unwrap();
    match list_value {
        StateValue::Set(set) => assert!(set.is_empty()),
        _ => panic!("Invalid handle type."),
    }
}

#[test]
fn run_with_args() {
    let mut context = test::run_script_and_validate(
        vec![create("")],
        r#"out = set_new 1 "hello world" test"#,
        CommandValidation::Contains("out".to_string(), "handle:".to_string()),
    );

    let state = get_handles_sub_state(&mut context.state);
    let list_value = state.remove(context.variables.get("out").unwrap()).unwrap();
    match list_value {
        StateValue::Set(set) => assert_eq!(set.len(), 3),
        _ => panic!("Invalid handle type."),
    }
}
