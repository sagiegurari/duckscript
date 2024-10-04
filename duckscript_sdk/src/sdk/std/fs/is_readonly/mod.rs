use crate::utils::pckg;
use duckscript::types::command::{Command, CommandInvocationContext, CommandResult};
use std::fs;

#[cfg(test)]
#[path = "./mod_test.rs"]
mod mod_test;

#[derive(Clone)]
pub(crate) struct CommandImpl {
    package: String,
}

impl Command for CommandImpl {
    fn name(&self) -> String {
        pckg::concat(&self.package, "IsReadonly")
    }

    fn aliases(&self) -> Vec<String> {
        vec!["is_readonly".to_string()]
    }

    fn help(&self) -> String {
        include_str!("help.md").to_string()
    }

    fn clone_and_box(&self) -> Box<dyn Command> {
        Box::new((*self).clone())
    }

    fn run(&self, context: CommandInvocationContext) -> CommandResult {
        if context.arguments.is_empty() {
            CommandResult::Error("Path not provided.".to_string())
        } else {
            match fs::metadata(&context.arguments[0]) {
                Ok(metadata) => {
                    CommandResult::Continue(Some(metadata.permissions().readonly().to_string()))
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
