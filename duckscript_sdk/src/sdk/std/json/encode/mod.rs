use crate::sdk::std::json::OBJECT_VALUE;
use crate::utils::pckg;
use duckscript::types::command::{Command, CommandResult, Commands};
use duckscript::types::instruction::Instruction;
use duckscript::types::runtime::StateValue;
use serde_json::map::Map;
use serde_json::Value;
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

fn encode(name: &str, variables: &HashMap<String, String>) -> Result<String, String> {
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
        _state: &mut HashMap<String, StateValue>,
        variables: &mut HashMap<String, String>,
        _output_variable: Option<String>,
        _instructions: &Vec<Instruction>,
        _commands: &mut Commands,
        _line: usize,
    ) -> CommandResult {
        if arguments.is_empty() {
            CommandResult::Error("No JSON root variable name provided.".to_string())
        } else {
            match encode(&arguments[0], variables) {
                Ok(output) => CommandResult::Continue(Some(output)),
                Err(error) => CommandResult::Error(error),
            }
        }
    }
}

pub(crate) fn create(package: &str) -> Box<dyn Command> {
    Box::new(CommandImpl {
        package: package.to_string(),
    })
}
