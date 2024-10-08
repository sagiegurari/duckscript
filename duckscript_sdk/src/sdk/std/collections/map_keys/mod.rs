use crate::utils::pckg;
use crate::utils::state::{get_handles_sub_state, put_handle};
use duckscript::types::command::{Command, CommandInvocationContext, CommandResult};
use duckscript::types::runtime::StateValue;
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
        pckg::concat(&self.package, "MapKeys")
    }

    fn aliases(&self) -> Vec<String> {
        vec!["map_keys".to_string()]
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
            let handles_state = get_handles_sub_state(context.state);

            match handles_state.get(&context.arguments[0]) {
                Some(state_value) => match state_value {
                    StateValue::SubState(ref map) => {
                        let mut array = vec![];

                        for map_key in map.keys() {
                            array.push(StateValue::String(map_key.to_string()));
                        }

                        let key = put_handle(context.state, StateValue::List(array));

                        CommandResult::Continue(Some(key))
                    }
                    _ => CommandResult::Error("Invalid handle provided.".to_string()),
                },
                None => CommandResult::Error(
                    format!("Map for handle: {} not found.", &context.arguments[0]).to_string(),
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
