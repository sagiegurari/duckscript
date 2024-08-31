use crate::sdk::std::json::OBJECT_VALUE;
use crate::utils::pckg;
use crate::utils::state::put_handle;
use duckscript::types::command::{Command, CommandResult, Commands};
use duckscript::types::env::Env;
use duckscript::types::instruction::Instruction;
use duckscript::types::runtime::StateValue;
use serde_json::{Result, Value};
use std::collections::HashMap;

#[cfg(test)]
#[path = "./mod_test.rs"]
mod mod_test;

fn parse_json(data: &str) -> Result<Value> {
    let value: Value = serde_json::from_str(data)?;

    Ok(value)
}

fn create_variables(data: Value, name: &str, variables: &mut HashMap<String, String>) {
    match data {
        Value::Null => variables.remove(name),
        Value::Bool(value) => variables.insert(name.to_string(), value.to_string()),
        Value::Number(value) => variables.insert(name.to_string(), value.to_string()),
        Value::String(value) => variables.insert(name.to_string(), value),
        Value::Array(list) => {
            let mut index = 0;
            for item in list {
                let child_name = format!("{}[{}]", name, index);
                create_variables(item, &child_name, variables);
                index = index + 1;
            }
            variables.insert(format!("{}.length", name), index.to_string());

            None
        }
        Value::Object(map) => {
            variables.insert(name.to_string(), OBJECT_VALUE.to_string());

            for (key, value) in map {
                let child_name = format!("{}.{}", name, key);
                create_variables(value, &child_name, variables);
            }

            None
        }
    };
}

fn create_structure(data: Value, state: &mut HashMap<String, StateValue>) -> Option<String> {
    match data {
        Value::Null => None,
        Value::Bool(value) => Some(value.to_string()),
        Value::Number(value) => Some(value.to_string()),
        Value::String(value) => Some(value),
        Value::Array(list) => {
            let mut state_list = vec![];

            for item in list {
                if let Some(value) = create_structure(item, state) {
                    state_list.push(StateValue::String(value));
                }
            }

            let key = put_handle(state, StateValue::List(state_list));

            Some(key)
        }
        Value::Object(map) => {
            let mut state_map = HashMap::new();

            for (key, value) in map {
                if let Some(value) = create_structure(value, state) {
                    state_map.insert(key, StateValue::String(value));
                }
            }

            let key = put_handle(state, StateValue::SubState(state_map));

            Some(key)
        }
    }
}

#[derive(Clone)]
pub(crate) struct CommandImpl {
    package: String,
}

impl Command for CommandImpl {
    fn name(&self) -> String {
        pckg::concat(&self.package, "Parse")
    }

    fn aliases(&self) -> Vec<String> {
        vec!["json_parse".to_string()]
    }

    fn help(&self) -> String {
        include_str!("help.md").to_string()
    }

    fn clone_and_box(&self) -> Box<dyn Command> {
        Box::new((*self).clone())
    }

    fn requires_context(&self) -> bool {
        true
    }

    fn run_with_context(
        &self,
        arguments: Vec<String>,
        state: &mut HashMap<String, StateValue>,
        variables: &mut HashMap<String, String>,
        output_variable: Option<String>,
        _instructions: &Vec<Instruction>,
        _commands: &mut Commands,
        _line: usize,
        _env: &mut Env,
    ) -> CommandResult {
        if arguments.is_empty() {
            CommandResult::Error("No JSON string provided.".to_string())
        } else {
            let (json_index, as_state) = if arguments.len() > 1 && arguments[0] == "--collection" {
                (1, true)
            } else {
                (0, false)
            };

            match parse_json(&arguments[json_index]) {
                Ok(data) => {
                    let output = match output_variable {
                        Some(name) => {
                            if as_state {
                                create_structure(data, state)
                            } else {
                                create_variables(data, &name, variables);

                                match variables.get(&name) {
                                    Some(value) => Some(value.to_string()),
                                    None => None,
                                }
                            }
                        }
                        None => Some("true".to_string()),
                    };

                    CommandResult::Continue(output)
                }
                Err(error) => CommandResult::Error(error.to_string()),
            }
        }
    }
}

pub(crate) fn create(package: &str) -> Box<dyn Command> {
    Box::new(CommandImpl {
        package: package.to_string(),
    })
}
