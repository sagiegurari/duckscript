use crate::utils::pckg;
use crate::utils::state::get_handles_sub_state;
use duckscript::types::command::{Command, CommandResult, Commands};
use duckscript::types::instruction::Instruction;
use duckscript::types::runtime::StateValue;
use java_properties::read;
use std::collections::HashMap;

#[cfg(test)]
#[path = "./mod_test.rs"]
mod mod_test;

#[derive(Clone)]
pub(crate) struct CommandImpl {
    package: String,
}

impl Command for CommandImpl {
    fn name(&self) -> String {
        pckg::concat(&self.package, "MapLoadProperties")
    }

    fn aliases(&self) -> Vec<String> {
        vec!["map_load_properties".to_string()]
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
        _variables: &mut HashMap<String, String>,
        _output_variable: Option<String>,
        _instructions: &Vec<Instruction>,
        _commands: &mut Commands,
        _line: usize,
    ) -> CommandResult {
        if arguments.len() < 2 {
            CommandResult::Error("Map handle and/or properties text not provided.".to_string())
        } else {
            let (prefix, key, text) = if arguments.len() >= 4 && arguments[0] == "--prefix" {
                (
                    arguments[1].to_string(),
                    arguments[2].to_string(),
                    arguments[3].to_string(),
                )
            } else {
                (
                    "".to_string(),
                    arguments[0].to_string(),
                    arguments[1].to_string(),
                )
            };

            match read(text.as_bytes()) {
                Ok(data) => {
                    let state = get_handles_sub_state(state);

                    match state.remove(&key) {
                        Some(state_value) => match state_value {
                            StateValue::SubState(mut map) => {
                                for (property_key, property_value) in &data {
                                    let mut var_key = property_key.to_string();
                                    if !prefix.is_empty() {
                                        var_key.insert(0, '.');
                                        var_key.insert_str(0, &prefix);
                                    }

                                    map.insert(
                                        var_key,
                                        StateValue::String(property_value.to_string()),
                                    );
                                }

                                state.insert(key.to_string(), StateValue::SubState(map));

                                CommandResult::Continue(Some("true".to_string()))
                            }
                            StateValue::Boolean(value) => {
                                state.insert(key.to_string(), StateValue::Boolean(value));
                                CommandResult::Error("Invalid handle provided.".to_string())
                            }
                            StateValue::Number(value) => {
                                state.insert(key.to_string(), StateValue::Number(value));
                                CommandResult::Error("Invalid handle provided.".to_string())
                            }
                            StateValue::UnsignedNumber(value) => {
                                state.insert(key.to_string(), StateValue::UnsignedNumber(value));
                                CommandResult::Error("Invalid handle provided.".to_string())
                            }
                            StateValue::Number32Bit(value) => {
                                state.insert(key.to_string(), StateValue::Number32Bit(value));
                                CommandResult::Error("Invalid handle provided.".to_string())
                            }
                            StateValue::UnsignedNumber32Bit(value) => {
                                state.insert(
                                    key.to_string(),
                                    StateValue::UnsignedNumber32Bit(value),
                                );
                                CommandResult::Error("Invalid handle provided.".to_string())
                            }
                            StateValue::Number64Bit(value) => {
                                state.insert(key.to_string(), StateValue::Number64Bit(value));
                                CommandResult::Error("Invalid handle provided.".to_string())
                            }
                            StateValue::UnsignedNumber64Bit(value) => {
                                state.insert(
                                    key.to_string(),
                                    StateValue::UnsignedNumber64Bit(value),
                                );
                                CommandResult::Error("Invalid handle provided.".to_string())
                            }
                            StateValue::String(value) => {
                                state.insert(key.to_string(), StateValue::String(value));
                                CommandResult::Error("Invalid handle provided.".to_string())
                            }
                            StateValue::ByteArray(value) => {
                                state.insert(key.to_string(), StateValue::ByteArray(value));
                                CommandResult::Error("Invalid handle provided.".to_string())
                            }
                            StateValue::List(value) => {
                                state.insert(key.to_string(), StateValue::List(value));
                                CommandResult::Error("Invalid handle provided.".to_string())
                            }
                        },
                        None => CommandResult::Error(
                            format!("Map for handle: {} not found.", key).to_string(),
                        ),
                    }
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
