use crate::utils::pckg;
use crate::utils::state::get_handles_sub_state;
use duckscript::types::command::{Command, CommandArgs, CommandResult, Commands};
use duckscript::types::env::Env;
use duckscript::types::instruction::Instruction;
use duckscript::types::runtime::StateValue;
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
        pckg::concat(&self.package, "BytesToString")
    }

    fn aliases(&self) -> Vec<String> {
        vec!["bytes_to_string".to_string()]
    }

    fn help(&self) -> String {
        include_str!("help.md").to_string()
    }

    fn clone_and_box(&self) -> Box<dyn Command> {
        Box::new((*self).clone())
    }

    fn run(&self, arguments: CommandArgs) -> CommandResult {
        if arguments.args.is_empty() {
            CommandResult::Error("Array handle not provided.".to_string())
        } else {
            let state = get_handles_sub_state(state);

            let key = &arguments.args[0];

            match arguments.state.get(key) {
                Some(state_value) => match state_value {
                    StateValue::ByteArray(binary) => match str::from_utf8(&binary) {
                        Ok(text) => CommandResult::Continue(Some(text.to_string())),
                        Err(error) => CommandResult::Error(error.to_string()),
                    },
                    _ => CommandResult::Error("Invalid handle provided.".to_string()),
                },
                None => CommandResult::Error(
                    format!("Array for handle: {} not found.", key).to_string(),
                ),
            }
        }
    }
}

pub(crate) fn create(package: &str) -> Box<dyn Command> {
    Box::new(CommandImpl {
        package: package.to_string(),
    })
}
