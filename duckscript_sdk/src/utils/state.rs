use crate::utils::pckg;
use duckscript::types::runtime::StateValue;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use std::collections::HashMap;
use std::iter;

#[cfg(test)]
#[path = "./state_test.rs"]
mod state_test;

static HANDLE_SUB_STATE_KEY: &str = "handles";

pub(crate) fn get_core_sub_state_for_command(
    state: &mut HashMap<String, StateValue>,
    name: String,
) -> &mut HashMap<String, StateValue> {
    let sub_state_name = pckg::concat("duckscriptsdk", &name);

    get_sub_state(sub_state_name, state)
}

pub(crate) fn get_handles_sub_state(
    state: &mut HashMap<String, StateValue>,
) -> &mut HashMap<String, StateValue> {
    get_sub_state(HANDLE_SUB_STATE_KEY.to_string(), state)
}

pub(crate) fn put_handle(state: &mut HashMap<String, StateValue>, value: StateValue) -> String {
    // generate unique key
    let mut rng = thread_rng();
    let mut key: String = iter::repeat(())
        .map(|()| rng.sample(Alphanumeric))
        .take(20)
        .collect();
    key.insert_str(0, "handle:");

    let handle_state = get_handles_sub_state(state);

    handle_state.insert(key.clone(), value);

    key
}

pub(crate) fn get_handle(
    state: &mut HashMap<String, StateValue>,
    key: String,
) -> Option<&StateValue> {
    let handle_state = get_handles_sub_state(state);

    handle_state.get(&key)
}

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

fn ensure_list(key: &str, state: &mut HashMap<String, StateValue>) {
    match state.get(key) {
        Some(value) => match value {
            StateValue::List(_) => (),
            _ => {
                state.insert(key.to_string(), StateValue::List(vec![]));
                ()
            }
        },
        None => {
            state.insert(key.to_string(), StateValue::List(vec![]));
            ()
        }
    }
}

/// This function will return a list from the context state.
/// If the list doesn't exist, it will create it first.
/// If the list is of different type, the old value will be deleted.
pub(crate) fn get_list(
    key: String,
    state: &mut HashMap<String, StateValue>,
) -> &mut Vec<StateValue> {
    ensure_list(&key, state);

    match state.get_mut(&key) {
        Some(value) => match value {
            StateValue::List(ref mut list) => list,
            _ => panic!("Internal state corrupted, invalid type."),
        },
        None => panic!("Internal state corrupted, missing data."),
    }
}
