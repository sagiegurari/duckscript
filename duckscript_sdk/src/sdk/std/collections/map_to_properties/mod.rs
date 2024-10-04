use crate::utils::pckg;
use crate::utils::state::{get_as_string, get_handles_sub_state};
use duckscript::types::command::{Command, CommandInvocationContext, CommandResult};
use duckscript::types::runtime::StateValue;
use java_properties::write;
use std::collections::HashMap;
use std::str;

#[cfg(test)]
#[path = "./mod_test.rs"]
mod mod_test;

#[derive(Clone)]
pub(crate) struct CommandImpl {
    package: String,
}

impl Command for CommandImpl {
    fn name(&self) -> String {
        pckg::concat(&self.package, "MapToProperties")
    }

    fn aliases(&self) -> Vec<String> {
        vec!["map_to_properties".to_string()]
    }

    fn help(&self) -> String {
        include_str!("help.md").to_string()
    }

    fn clone_and_box(&self) -> Box<dyn Command> {
        Box::new((*self).clone())
    }

    fn run(&self, context: CommandInvocationContext) -> CommandResult {
        if context.arguments.is_empty() {
            CommandResult::Error("Map handle not provided.".to_string())
        } else {
            let (prefix, key) =
                if context.arguments.len() >= 3 && context.arguments[0] == "--prefix" {
                    (
                        context.arguments[1].to_string(),
                        context.arguments[2].to_string(),
                    )
                } else {
                    ("".to_string(), context.arguments[0].to_string())
                };

            let state = get_handles_sub_state(context.state);

            match state.get(&key) {
                Some(state_value) => match state_value {
                    StateValue::SubState(ref map) => {
                        let mut properties = HashMap::new();

                        for (property_key, property_value) in map {
                            let mut var_key = property_key.to_string();
                            if !prefix.is_empty() {
                                var_key.insert(0, '.');
                                var_key.insert_str(0, &prefix);
                            }

                            let string_value = match get_as_string(property_value) {
                                Ok(value) => value,
                                Err(error) => return CommandResult::Error(error),
                            };

                            properties.insert(var_key, string_value);
                        }

                        let mut buffer: Vec<u8> = vec![];
                        match write(&mut buffer, &properties) {
                            Ok(_) => match str::from_utf8(&buffer) {
                                Ok(text) => CommandResult::Continue(Some(text.trim().to_string())),
                                Err(error) => CommandResult::Error(error.to_string()),
                            },
                            Err(error) => CommandResult::Error(error.to_string()),
                        }
                    }
                    _ => CommandResult::Error("Invalid handle provided.".to_string()),
                },
                None => {
                    CommandResult::Error(format!("Map for handle: {} not found.", key).to_string())
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
