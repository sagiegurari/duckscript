use crate::utils::pckg;
use crate::utils::state::{get_handles_sub_state, get_optional_as_string, mutate_map};
use duckscript::types::command::{Command, CommandInvocationContext, CommandResult};

#[cfg(test)]
#[path = "./mod_test.rs"]
mod mod_test;

#[derive(Clone)]
pub(crate) struct CommandImpl {
    package: String,
}

impl Command for CommandImpl {
    fn name(&self) -> String {
        pckg::concat(&self.package, "MapGet")
    }

    fn aliases(&self) -> Vec<String> {
        vec!["map_get".to_string()]
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
        } else if context.arguments.len() < 2 {
            CommandResult::Error("Key not provided.".to_string())
        } else {
            let state = get_handles_sub_state(context.state);

            let key = context.arguments[0].clone();

            let result = mutate_map(key, state, |map| {
                let item = match map.remove(&context.arguments[1]) {
                    Some(value) => {
                        let value_clone = value.clone();
                        map.insert(context.arguments[1].clone(), value);
                        Some(value_clone)
                    }
                    None => None,
                };

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
