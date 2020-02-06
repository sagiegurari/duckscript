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

pub(crate) fn get_optional_as_string(
    state_value_option: Option<StateValue>,
) -> Result<Option<String>, String> {
    match state_value_option {
        Some(ref state_value) => match get_as_string(state_value) {
            Ok(value) => Ok(Some(value)),
            Err(error) => Err(error),
        },
        None => Ok(None),
    }
}

pub(crate) fn get_as_string(state_value: &StateValue) -> Result<String, String> {
    match state_value {
        StateValue::Boolean(value) => Ok(value.to_string()),
        StateValue::Number(value) => Ok(value.to_string()),
        StateValue::UnsignedNumber(value) => Ok(value.to_string()),
        StateValue::Number32Bit(value) => Ok(value.to_string()),
        StateValue::UnsignedNumber32Bit(value) => Ok(value.to_string()),
        StateValue::Number64Bit(value) => Ok(value.to_string()),
        StateValue::UnsignedNumber64Bit(value) => Ok(value.to_string()),
        StateValue::String(value) => Ok(value.to_string()),
        StateValue::ByteArray(_) => Err("Unsupported value type.".to_string()),
        StateValue::List(_) => Err("Unsupported value type.".to_string()),
        StateValue::SubState(_) => Err("Unsupported value type.".to_string()),
    }
}

pub(crate) fn mutate_map<F>(
    key: String,
    state: &mut HashMap<String, StateValue>,
    mut handler: F,
) -> Result<Option<String>, String>
where
    F: FnMut(&mut HashMap<String, StateValue>) -> Result<Option<String>, String>,
{
    match state.remove(&key) {
        Some(state_value) => match state_value {
            StateValue::SubState(mut map) => {
                let result = handler(&mut map);

                state.insert(key, StateValue::SubState(map));

                result
            }
            StateValue::Boolean(value) => {
                state.insert(key, StateValue::Boolean(value));
                Err("Invalid handle provided.".to_string())
            }
            StateValue::Number(value) => {
                state.insert(key, StateValue::Number(value));
                Err("Invalid handle provided.".to_string())
            }
            StateValue::UnsignedNumber(value) => {
                state.insert(key, StateValue::UnsignedNumber(value));
                Err("Invalid handle provided.".to_string())
            }
            StateValue::Number32Bit(value) => {
                state.insert(key, StateValue::Number32Bit(value));
                Err("Invalid handle provided.".to_string())
            }
            StateValue::UnsignedNumber32Bit(value) => {
                state.insert(key, StateValue::UnsignedNumber32Bit(value));
                Err("Invalid handle provided.".to_string())
            }
            StateValue::Number64Bit(value) => {
                state.insert(key, StateValue::Number64Bit(value));
                Err("Invalid handle provided.".to_string())
            }
            StateValue::UnsignedNumber64Bit(value) => {
                state.insert(key, StateValue::UnsignedNumber64Bit(value));
                Err("Invalid handle provided.".to_string())
            }
            StateValue::String(value) => {
                state.insert(key, StateValue::String(value));
                Err("Invalid handle provided.".to_string())
            }
            StateValue::ByteArray(value) => {
                state.insert(key, StateValue::ByteArray(value));
                Err("Invalid handle provided.".to_string())
            }
            StateValue::List(value) => {
                state.insert(key, StateValue::List(value));
                Err("Invalid handle provided.".to_string())
            }
        },
        None => Err(format!("Handle: {} not found.", &key).to_string()),
    }
}

pub(crate) fn mutate_list<F>(
    key: String,
    state: &mut HashMap<String, StateValue>,
    mut handler: F,
) -> Result<Option<String>, String>
where
    F: FnMut(&mut Vec<StateValue>) -> Result<Option<String>, String>,
{
    match state.remove(&key) {
        Some(state_value) => match state_value {
            StateValue::List(mut list) => {
                let result = handler(&mut list);

                state.insert(key, StateValue::List(list));

                result
            }
            StateValue::Boolean(value) => {
                state.insert(key, StateValue::Boolean(value));
                Err("Invalid handle provided.".to_string())
            }
            StateValue::Number(value) => {
                state.insert(key, StateValue::Number(value));
                Err("Invalid handle provided.".to_string())
            }
            StateValue::UnsignedNumber(value) => {
                state.insert(key, StateValue::UnsignedNumber(value));
                Err("Invalid handle provided.".to_string())
            }
            StateValue::Number32Bit(value) => {
                state.insert(key, StateValue::Number32Bit(value));
                Err("Invalid handle provided.".to_string())
            }
            StateValue::UnsignedNumber32Bit(value) => {
                state.insert(key, StateValue::UnsignedNumber32Bit(value));
                Err("Invalid handle provided.".to_string())
            }
            StateValue::Number64Bit(value) => {
                state.insert(key, StateValue::Number64Bit(value));
                Err("Invalid handle provided.".to_string())
            }
            StateValue::UnsignedNumber64Bit(value) => {
                state.insert(key, StateValue::UnsignedNumber64Bit(value));
                Err("Invalid handle provided.".to_string())
            }
            StateValue::String(value) => {
                state.insert(key, StateValue::String(value));
                Err("Invalid handle provided.".to_string())
            }
            StateValue::ByteArray(value) => {
                state.insert(key, StateValue::ByteArray(value));
                Err("Invalid handle provided.".to_string())
            }
            StateValue::SubState(value) => {
                state.insert(key, StateValue::SubState(value));
                Err("Invalid handle provided.".to_string())
            }
        },
        None => Err(format!("Handle: {} not found.", &key).to_string()),
    }
}
