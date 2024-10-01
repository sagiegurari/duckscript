use crate::utils::pckg;
use crate::utils::state::{get_handles_sub_state, get_optional_as_string, mutate_map};
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
        pckg::concat(&self.package, "MapRemove")
    }

    fn aliases(&self) -> Vec<String> {
        vec!["map_remove".to_string()]
    }

    fn help(&self) -> String {
        include_str!("help.md").to_string()
    }

    fn clone_and_box(&self) -> Box<dyn Command> {
        Box::new((*self).clone())
    }

    fn run(&self, arguments: CommandArgs) -> CommandResult {
        if arguments.args.is_empty() {
            CommandResult::Error("Map handle not provided.".to_string())
        } else if arguments.args.len() < 2 {
            CommandResult::Error("Key not provided.".to_string())
        } else {
            let state = get_handles_sub_state(arguments.state);

            let key = arguments.args[0].clone();

            let result = mutate_map(key, state, |map| {
                let item = map.remove(&arguments.args[1]);

                get_optional_as_string(item)
            });

            match result {
                Ok(value) => CommandResult::Continue(value),
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
