use crate::utils::pckg;
use crate::utils::state::{remove_handle, remove_handle_recursive};
use duckscript::types::command::{Command, CommandArgs, CommandResult, Commands};
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

    fn run(&self, arguments: CommandArgs) -> CommandResult {
        if arguments.args.is_empty() {
            CommandResult::Continue(Some("false".to_string()))
        } else {
            let (key, recursive) = if arguments.args.len() > 1
                && (arguments.args[0] == "-r" || arguments.args[0] == "--recursive")
            {
                (arguments.args[1].to_string(), true)
            } else {
                (arguments.args[0].to_string(), false)
            };

            let removed = if recursive {
                remove_handle_recursive(arguments.state, key)
            } else {
                let old_value = remove_handle(arguments.state, key);
                old_value.is_some()
            };

            CommandResult::Continue(Some(removed.to_string()))
        }
    }
}

pub(crate) fn create(package: &str) -> Box<dyn Command> {
    Box::new(CommandImpl {
        package: package.to_string(),
    })
}
