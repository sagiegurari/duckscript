use crate::sdk::std::json::OBJECT_VALUE;
use crate::utils::pckg;
use crate::utils::state::get_handles_sub_state;
use duckscript::types::command::{Command, CommandResult, Commands};
use duckscript::types::env::Env;
use duckscript::types::instruction::Instruction;
use duckscript::types::runtime::StateValue;
use serde_json::map::Map;
use serde_json::{Number, Value};
use std::collections::{HashMap, HashSet};

#[cfg(test)]
#[path = "./mod_test.rs"]
mod mod_test;

fn encode_array(name: &str, values: &HashMap<&str, &str>) -> Result<Value, String> {
    match values.get(format!("{}.length", name).as_str()) {
        Some(length_str) => match length_str.parse::<usize>() {
            Ok(length) => {
                let mut json_vec = vec![];

                for index in 0..length {
                    let array_item_name = format!("{}[{}]", name, index);
                    match encode_any(&array_item_name, values) {
                        Ok(array_item) => json_vec.push(array_item),
                        Err(error) => return Err(error),
                    }
                }

                Ok(Value::Array(json_vec))
            }
            Err(error) => Err(error.to_string()),
        },
        None => Err(format!(
            "{} is not a valid JSON array, missing length attribute.",
            name
        )),
    }
}

fn encode_object(name: &str, values: &HashMap<&str, &str>) -> Result<Value, String> {
    let child_prefix = format!("{}.", name);
    let child_prefix_end = child_prefix.len() - 1;
    let mut children: HashSet<&str> = HashSet::new();

    for (key, _) in values {
        if key.starts_with(&child_prefix) {
            let last_index = key.rfind('.').unwrap();

            if last_index == child_prefix_end {
                let array_key_end = key.rfind("[").unwrap_or(0);
                let next_key = if array_key_end > last_index && key.rfind("]").is_some() {
                    &key[0..array_key_end]
                } else {
                    key
                };
                children.insert(next_key);
            }
        }
    }

    let mut object = Map::new();
    let prefix_length = name.len() + 1;
    for key in children {
        match encode_any(key, values) {
            Ok(json_value) => {
                let child_key = &key[prefix_length..];
                object.insert(child_key.to_string(), json_value);
                ()
            }
            Err(error) => return Err(error),
        }
    }

    Ok(Value::Object(object))
}

fn encode_any(name: &str, values: &HashMap<&str, &str>) -> Result<Value, String> {
    match values.get(name) {
        Some(value) => {
            if *value == OBJECT_VALUE {
                encode_object(name, values)
            } else {
                Ok(Value::String(value.to_string()))
            }
        }
        None => {
            if values.contains_key(format!("{}.length", name).as_str()) {
                encode_array(name, values)
            } else {
                Ok(Value::Null)
            }
        }
    }
}

fn encode_from_variables(
    name: &str,
    variables: &HashMap<String, String>,
) -> Result<String, String> {
    let mut object_variables: HashMap<&str, &str> = HashMap::new();

    for (key, value) in variables {
        if key == name || key.starts_with(name) {
            object_variables.insert(key, value);
        }
    }

    if object_variables.is_empty() {
        Ok("".to_string())
    } else {
        match encode_any(name, &object_variables) {
            Ok(value) => Ok(value.to_string()),
            Err(error) => Err(error),
        }
    }
}

fn encode_from_state_value(
    state_value: &StateValue,
    state: &HashMap<String, StateValue>,
) -> Result<Value, String> {
    match state_value {
        StateValue::Boolean(value) => Ok(Value::Bool(*value)),
        StateValue::Number(value) => Ok(Value::Number(Number::from(*value))),
        StateValue::UnsignedNumber(value) => Ok(Value::Number(Number::from(*value))),
        StateValue::Number32Bit(value) => Ok(Value::Number(Number::from(*value))),
        StateValue::UnsignedNumber32Bit(value) => Ok(Value::Number(Number::from(*value))),
        StateValue::Number64Bit(value) => Ok(Value::Number(Number::from(*value))),
        StateValue::UnsignedNumber64Bit(value) => Ok(Value::Number(Number::from(*value))),
        StateValue::String(value) => match state.get(value) {
            Some(sub_state_value) => encode_from_state_value(sub_state_value, state),
            None => Ok(Value::String(value.to_string())),
        },
        StateValue::List(list) => {
            let mut items = vec![];

            for item in list {
                match encode_from_state_value(item, state) {
                    Ok(item_value) => {
                        items.push(item_value);
                    }
                    Err(error) => return Err(error),
                };
            }

            Ok(Value::Array(items))
        }
        StateValue::SubState(sub_state) => {
            let mut items = Map::new();

            for (key, value) in sub_state {
                match encode_from_state_value(value, state) {
                    Ok(item_value) => {
                        items.insert(key.to_string(), item_value);
                    }
                    Err(error) => return Err(error),
                }
            }

            Ok(Value::Object(items))
        }
        StateValue::ByteArray(_) => Err("Unsupported value type.".to_string()),
        StateValue::Set(_) => Err("Unsupported value type.".to_string()),
        StateValue::Any(_) => Err("Unsupported value type.".to_string()),
    }
}

fn encode_from_state(value: &str, state: &HashMap<String, StateValue>) -> Result<String, String> {
    let json_value = match state.get(value) {
        Some(state_value) => encode_from_state_value(state_value, state),
        None => Ok(Value::String(value.to_string())),
    };

    match json_value {
        Ok(json_value_obj) => Ok(json_value_obj.to_string()),
        Err(error) => Err(error.to_string()),
    }
}

#[derive(Clone)]
pub(crate) struct CommandImpl {
    package: String,
}

impl Command for CommandImpl {
    fn name(&self) -> String {
        pckg::concat(&self.package, "Encode")
    }

    fn aliases(&self) -> Vec<String> {
        vec!["json_encode".to_string()]
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
        _output_variable: Option<String>,
        _instructions: &Vec<Instruction>,
        _commands: &mut Commands,
        _line: usize,
        _env: &mut Env,
    ) -> CommandResult {
        if arguments.is_empty() {
            CommandResult::Error("No JSON root variable name provided.".to_string())
        } else {
            let (start_index, as_state) = if arguments.len() > 1 && arguments[0] == "--collection" {
                (1, true)
            } else {
                (0, false)
            };

            if as_state {
                let state = get_handles_sub_state(state);

                match encode_from_state(&arguments[start_index], state) {
                    Ok(output) => CommandResult::Continue(Some(output)),
                    Err(error) => CommandResult::Error(error),
                }
            } else {
                match encode_from_variables(&arguments[start_index], variables) {
                    Ok(output) => CommandResult::Continue(Some(output)),
                    Err(error) => CommandResult::Error(error),
                }
            }
        }
    }
}

pub(crate) fn create(package: &str) -> Box<dyn Command> {
    Box::new(CommandImpl {
        package: package.to_string(),
    })
}
