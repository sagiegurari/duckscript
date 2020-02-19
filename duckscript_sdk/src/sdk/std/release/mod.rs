use crate::utils::pckg;
use crate::utils::state::remove_handle;
use duckscript::types::command::{Command, CommandResult, Commands};
use duckscript::types::instruction::Instruction;
use duckscript::types::runtime::StateValue;
use std::collections::HashMap;

#[cfg(test)]
#[path = "./mod_test.rs"]
mod mod_test;

fn remove(state: &mut HashMap<String, StateValue>, key: &str, recursive: bool) -> bool {
    match remove_handle(state, key.to_string()) {
        Some(state_value) => {
            if recursive {
                match state_value {
                    StateValue::SubState(map) => {
                        for (_, map_value) in map {
                            match map_value {
                                StateValue::String(value) => remove(state, &value, recursive),
                                _ => true,
                            };
                        }

                        true
                    }
                    StateValue::List(list) => {
                        for value in list {
                            match value {
                                StateValue::String(value) => remove(state, &value, recursive),
                                _ => true,
                            };
                        }

                        true
                    }
                    _ => true,
                }
            } else {
                true
            }
        }
        None => false,
    }
}

#[derive(Clone)]
pub(crate) struct CommandImpl {
    package: String,
}

impl Command for CommandImpl {
    fn name(&self) -> String {
        pckg::concat(&self.package, "Release")
    }

    fn aliases(&self) -> Vec<String> {
        vec!["release".to_string()]
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
        if arguments.is_empty() {
            CommandResult::Continue(Some("false".to_string()))
        } else {
            let (key, recursive) =
                if arguments.len() > 1 && (arguments[0] == "-r" || arguments[0] == "--recursive") {
                    (arguments[1].to_string(), true)
                } else {
                    (arguments[0].to_string(), false)
                };

            let removed = remove(state, &key, recursive);

            CommandResult::Continue(Some(removed.to_string()))
        }
    }
}

pub(crate) fn create(package: &str) -> Box<dyn Command> {
    Box::new(CommandImpl {
        package: package.to_string(),
    })
}
