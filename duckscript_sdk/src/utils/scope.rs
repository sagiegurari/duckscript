use crate::utils::state::{ensure_list, mutate_list};
use duckscript::types::runtime::StateValue;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

static SCOPE_STACK_STATE_KEY: &str = "scope_stack";

pub(crate) fn push(
    variables: &mut HashMap<String, String>,
    state: &mut HashMap<String, StateValue>,
    copy: &[String],
) -> Result<(), String> {
    ensure_list(SCOPE_STACK_STATE_KEY, state);

    match mutate_list(SCOPE_STACK_STATE_KEY.to_string(), state, |list| {
        list.push(StateValue::Any(Rc::new(RefCell::new(variables.clone()))));
        Ok(None)
    }) {
        Ok(_) => {
            let mut new_variables = HashMap::new();
            for key in copy {
                let value = variables.remove(key);
                new_variables.insert(key, value);
            }

            variables.clear();

            for (key, value) in new_variables {
                variables.insert(key.to_string(), value.unwrap());
            }

            Ok(())
        }
        Err(error) => Err(error),
    }
}

pub(crate) fn pop(
    variables: &mut HashMap<String, String>,
    state: &mut HashMap<String, StateValue>,
    copy: &[String],
) -> Result<(), String> {
    ensure_list(SCOPE_STACK_STATE_KEY, state);

    match mutate_list(
        SCOPE_STACK_STATE_KEY.to_string(),
        state,
        |list| match list.pop() {
            Some(state_value) => match state_value {
                StateValue::Any(rc_value) => {
                    let value_any = rc_value.borrow();

                    match value_any.downcast_ref::<HashMap<String, String>>() {
                        Some(old_variables) => {
                            let mut new_variables = HashMap::new();
                            for key in copy {
                                let value = variables.remove(key);
                                new_variables.insert(key, value);
                            }

                            variables.clear();

                            for (key, value) in old_variables {
                                variables.insert(key.to_string(), value.clone());
                            }

                            for (key, value) in new_variables {
                                variables.insert(key.to_string(), value.unwrap());
                            }

                            Ok(None)
                        }
                        None => Err("Scope stack not available, invalid type.".to_string()),
                    }
                }
                _ => Err("Scope stack not available, invalid state value type.".to_string()),
            },
            None => Err("Reached end of scope stack.".to_string()),
        },
    ) {
        Ok(_) => Ok(()),
        Err(error) => Err(error),
    }
}
