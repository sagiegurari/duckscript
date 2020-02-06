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
    test::run_script_and_error(vec![create("")], "out = split", "out");
}

#[test]
fn run_single_arg() {
    test::run_script_and_error(vec![create("")], "out = split abc", "out");
}

#[test]
fn run_split() {
    let mut context = test::run_script_and_validate(
        vec![create("")],
        r#"out = split "a b c" " ""#,
        CommandValidation::Ignore,
    );

    let state = get_handles_sub_state(&mut context.state);
    let list_value = state.remove(context.variables.get("out").unwrap()).unwrap();
    match list_value {
        StateValue::List(mut list) => {
            assert_eq!(list.len(), 3);

            match list.pop().unwrap() {
                StateValue::String(value) => assert_eq!(value, "c"),
                _ => panic!("Invalid handle value."),
            };

            match list.pop().unwrap() {
                StateValue::String(value) => assert_eq!(value, "b"),
                _ => panic!("Invalid handle value."),
            };

            match list.pop().unwrap() {
                StateValue::String(value) => assert_eq!(value, "a"),
                _ => panic!("Invalid handle value."),
            };
        }
        _ => panic!("Invalid type."),
    }
}
