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
        "out = map",
        CommandValidation::Contains("out".to_string(), "handle:".to_string()),
    );

    let state = get_handles_sub_state(&mut context.state);
    let map_value = state.get(context.variables.get("out").unwrap()).unwrap();
    match map_value {
        StateValue::SubState(map) => assert!(map.is_empty()),
        _ => panic!("Invalid handle type."),
    }
}
