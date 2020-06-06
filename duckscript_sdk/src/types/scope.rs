use crate::utils::state::get_core_sub_state_for_runtime;
use duckscript::types::runtime::StateValue;
use std::collections::HashMap;

static CONTEXT_NAME_SUB_STATE_KEY: &str = "line_context_name";
static CONTEXT_NAME_STATE_KEY: &str = "name";

pub(crate) fn get_line_context_name(state: &mut HashMap<String, StateValue>) -> String {
    let sub_state = get_core_sub_state_for_runtime(state, CONTEXT_NAME_SUB_STATE_KEY.to_string());

    match sub_state.get(CONTEXT_NAME_STATE_KEY) {
        Some(state_value) => match state_value {
            StateValue::String(value) => value.clone(),
            _ => {
                // remove corrupted data
                sub_state.remove(CONTEXT_NAME_STATE_KEY);

                "".to_string()
            }
        },
        None => "".to_string(),
    }
}

pub(crate) fn set_line_context_name(name: &str, state: &mut HashMap<String, StateValue>) -> String {
    let previous_name = get_line_context_name(state);

    let sub_state = get_core_sub_state_for_runtime(state, CONTEXT_NAME_SUB_STATE_KEY.to_string());
    sub_state.insert(
        CONTEXT_NAME_STATE_KEY.to_string(),
        StateValue::String(name.to_string()),
    );

    previous_name
}

pub(crate) fn clear(name: &str, variables: &mut HashMap<String, String>) {
    let mut scope_name = name.to_string();
    scope_name.push_str("::");

    variables.retain(|key, _| !key.starts_with(&scope_name));
}
