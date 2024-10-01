use crate::utils::pckg;
use crate::utils::state::{get_handles_sub_state, get_optional_as_string, mutate_list};
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
        pckg::concat(&self.package, "ArrayGet")
    }

    fn aliases(&self) -> Vec<String> {
        vec!["array_get".to_string()]
    }

    fn help(&self) -> String {
        include_str!("help.md").to_string()
    }

    fn clone_and_box(&self) -> Box<dyn Command> {
        Box::new((*self).clone())
    }

    fn run(&self, arguments: CommandArgs) -> CommandResult {
        if arguments.args.len() < 2 {
            CommandResult::Error("Array handle or item index not provided.".to_string())
        } else {
            let state = get_handles_sub_state(arguments.state);

            let key = arguments.args[0].clone();
            let index: usize = match arguments.args[1].parse() {
                Ok(value) => value,
                Err(_) => {
                    return CommandResult::Error(
                        format!("Non numeric value: {} provided.", &arguments.args[1]).to_string(),
                    );
                }
            };

            let result = mutate_list(key, state, |list| {
                if list.len() > index {
                    let list_item = list[index].clone();

                    get_optional_as_string(Some(list_item))
                } else {
                    Ok(None)
                }
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
