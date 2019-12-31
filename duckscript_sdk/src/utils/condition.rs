use crate::utils::eval;
use duckscript::types::command::{CommandResult, Commands};
use duckscript::types::runtime::StateValue;
use std::collections::HashMap;

#[cfg(test)]
#[path = "./condition_test.rs"]
mod condition_test;

pub(crate) fn is_true(value: Option<String>) -> bool {
    let failed = match value {
        Some(value_str) => {
            let lower_case = value_str.to_lowercase();
            lower_case == "" || lower_case == "0" || lower_case == "false" || lower_case == "no"
        }
        None => true,
    };

    !failed
}

pub(crate) fn eval_condition(
    arguments: Vec<String>,
    state: &mut HashMap<String, StateValue>,
    variables: &mut HashMap<String, String>,
    commands: &mut Commands,
) -> Result<bool, String> {
    let eval_statement = if arguments.len() == 1 {
        match commands.get(&arguments[0]) {
            Some(_) => true,
            None => false,
        }
    } else {
        true
    };

    if eval_statement {
        match eval::eval(&arguments, state, variables, commands) {
            CommandResult::Continue(value) => {
                let passed = is_true(value);

                Ok(passed)
            }
            CommandResult::Error(error) => Err(error.to_string()),
            _ => Err("Invalid condition evaluation result.".to_string()),
        }
    } else {
        let value = match variables.get(&arguments[0]) {
            Some(value) => Some(value.to_string()),
            None => None,
        };

        let passed = is_true(value);

        Ok(passed)
    }
}
