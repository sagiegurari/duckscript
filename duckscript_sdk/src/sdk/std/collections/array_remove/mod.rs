use crate::utils::pckg;
use crate::utils::state::{get_handles_sub_state, mutate_list};
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
        pckg::concat(&self.package, "ArrayRemove")
    }

    fn aliases(&self) -> Vec<String> {
        vec!["array_remove".to_string()]
    }

    fn help(&self) -> String {
        include_str!("help.md").to_string()
    }

    fn clone_and_box(&self) -> Box<dyn Command> {
        Box::new((*self).clone())
    }

    fn run(&self, context: CommandInvocationContext) -> CommandResult {
        if context.arguments.len() < 2 {
            CommandResult::Error("Invalid input provided.".to_string())
        } else {
            let state = get_handles_sub_state(context.state);

            let key = context.arguments[0].clone();
            let index: usize = match context.arguments[1].parse() {
                Ok(value) => value,
                Err(_) => {
                    return CommandResult::Error(
                        format!("Non numeric value: {} provided.", &context.arguments[1])
                            .to_string(),
                    );
                }
            };

            let result = mutate_list(key, state, |list| {
                let list_length = list.len();

                if list_length > index {
                    list.remove(index);

                    Ok(Some("true".to_string()))
                } else {
                    Err(format!(
                        "Index: {} is greater than list size: {}",
                        index, list_length
                    ))
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
