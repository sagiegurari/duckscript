use duckscript::types::runtime::StateValue;
use std::collections::HashMap;

#[cfg(test)]
#[path = "./state_test.rs"]
mod state_test;

fn ensure_sub_state(key: &str, state: &mut HashMap<String, StateValue>) {
    match state.get(key) {
        Some(value) => match value {
            StateValue::SubState(_) => (),
            _ => {
                state.insert(key.to_string(), StateValue::SubState(HashMap::new()));
                ()
            }
        },
        None => {
            state.insert(key.to_string(), StateValue::SubState(HashMap::new()));
            ()
        }
    }
}

/// This function will return a sub state from the context state.
/// If the sub state doesn't exist, it will create it first.
/// If the sub state is of different type, the old value will be deleted.
pub(crate) fn get_sub_state(
    key: String,
    state: &mut HashMap<String, StateValue>,
) -> &mut HashMap<String, StateValue> {
    ensure_sub_state(&key, state);

    match state.get_mut(&key) {
        Some(value) => match value {
            StateValue::SubState(ref mut sub_state) => sub_state,
            _ => panic!("Internal state corrupted, invalid type."),
        },
        None => panic!("Internal state corrupted, missing data."),
    }
}
