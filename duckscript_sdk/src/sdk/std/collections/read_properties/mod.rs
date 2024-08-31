use crate::utils::pckg;
use duckscript::types::command::{Command, CommandResult, Commands};
use duckscript::types::env::Env;
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
        pckg::concat(&self.package, "ReadProperties")
    }

    fn aliases(&self) -> Vec<String> {
        vec!["read_properties".to_string()]
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
        _env: &mut Env,
    ) -> CommandResult {
        if arguments.len() < 1 {
            CommandResult::Error("Missing properties text argument.".to_string())
        } else {
            let (prefix, text) = if arguments.len() >= 3 && arguments[0] == "--prefix" {
                (arguments[1].to_string(), arguments[2].to_string())
            } else {
                ("".to_string(), arguments[0].to_string())
            };

            match read(text.as_bytes()) {
                Ok(data) => {
                    for (key, value) in &data {
                        let mut var_key = key.to_string();
                        if !prefix.is_empty() {
                            var_key.insert(0, '.');
                            var_key.insert_str(0, &prefix);
                        }

                        variables.insert(var_key, value.to_string());
                    }

                    CommandResult::Continue(Some(data.len().to_string()))
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
