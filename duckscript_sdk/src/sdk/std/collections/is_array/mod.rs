use crate::utils::pckg;
use crate::utils::state::get_handles_sub_state;
use duckscript::types::command::{Command, CommandArgs, CommandResult};
use duckscript::types::env::Env;
use duckscript::types::instruction::Instruction;
use duckscript::types::runtime::StateValue;
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
        pckg::concat(&self.package, "IsArray")
    }

    fn aliases(&self) -> Vec<String> {
        vec!["is_array".to_string()]
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

            match state.get(key) {
                Some(state_value) => match state_value {
                    StateValue::List(_) => CommandResult::Continue(Some("true".to_string())),
                    _ => CommandResult::Continue(Some("false".to_string())),
                },
                None => CommandResult::Continue(Some("false".to_string())),
            }
        }
    }
}

pub(crate) fn create(package: &str) -> Box<dyn Command> {
    Box::new(CommandImpl {
        package: package.to_string(),
    })
}
