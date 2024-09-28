use crate::utils::pckg;
use crate::utils::state::put_handle;
use duckscript::types::command::{Command, CommandArgs, CommandResult, Commands};
use duckscript::types::env::Env;
use duckscript::types::instruction::Instruction;
use duckscript::types::runtime::StateValue;
use fsio::path::from_path::FromPath;
use glob::glob;
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
        pckg::concat(&self.package, "GlobArray")
    }

    fn aliases(&self) -> Vec<String> {
        vec!["glob_array".to_string(), "globarray".to_string()]
    }

    fn help(&self) -> String {
        include_str!("help.md").to_string()
    }

    fn clone_and_box(&self) -> Box<dyn Command> {
        Box::new((*self).clone())
    }

    fn run(&self, arguments: CommandArgs) -> CommandResult {
        if arguments.args.is_empty() {
            CommandResult::Error("Glob pattern not provided.".to_string())
        } else {
            match glob(&arguments.args[0]) {
                Ok(paths) => {
                    let mut array = vec![];

                    for entry in paths {
                        match entry {
                            Ok(path) => {
                                let value_string: String = FromPath::from_path(&path);
                                let state_value = StateValue::String(value_string);

                                array.push(state_value);
                            }
                            Err(error) => return CommandResult::Error(error.to_string()),
                        }
                    }

                    let key = put_handle(arguments.state, StateValue::List(array));

                    CommandResult::Continue(Some(key))
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
