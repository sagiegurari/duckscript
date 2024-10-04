use crate::utils::pckg;
use duckscript::types::command::{Command, CommandInvocationContext, CommandResult};
use std::thread;
use std::time::Duration;

#[cfg(test)]
#[path = "./mod_test.rs"]
mod mod_test;

#[derive(Clone)]
pub(crate) struct CommandImpl {
    package: String,
}

impl Command for CommandImpl {
    fn name(&self) -> String {
        pckg::concat(&self.package, "Sleep")
    }

    fn aliases(&self) -> Vec<String> {
        vec!["sleep".to_string()]
    }

    fn help(&self) -> String {
        include_str!("help.md").to_string()
    }

    fn clone_and_box(&self) -> Box<dyn Command> {
        Box::new((*self).clone())
    }

    fn run(&self, context: CommandInvocationContext) -> CommandResult {
        if context.arguments.is_empty() {
            CommandResult::Continue(Some("0".to_string()))
        } else {
            match context.arguments[0].parse() {
                Ok(value) => {
                    if value == 0 {
                        CommandResult::Continue(Some("0".to_string()))
                    } else if value > 0 {
                        let millis = Duration::from_millis(value);
                        thread::sleep(millis);
                        CommandResult::Continue(Some(value.to_string()))
                    } else {
                        CommandResult::Error(
                            format!("Negative value: {} provided.", value).to_string(),
                        )
                    }
                }
                Err(_) => CommandResult::Error(
                    format!("Non numeric value: {} provided.", &context.arguments[0]).to_string(),
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
