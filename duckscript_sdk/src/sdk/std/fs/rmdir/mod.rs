use crate::utils::pckg;
use duckscript::types::command::{Command, CommandResult};
use std::fs;

#[cfg(test)]
#[path = "./mod_test.rs"]
mod mod_test;

struct CommandImpl {
    package: String,
}

impl Command for CommandImpl {
    fn name(&self) -> String {
        pckg::concat(&self.package, "DeleteEmptyDirectory")
    }

    fn aliases(&self) -> Vec<String> {
        vec!["rmdir".to_string()]
    }

    fn help(&self) -> String {
        include_str!("help.md").to_string()
    }

    fn run(&self, arguments: Vec<String>) -> CommandResult {
        if arguments.is_empty() {
            CommandResult::Error("Directory path not provided.".to_string())
        } else {
            let result = fs::remove_dir(&arguments[0]);

            match result {
                Ok(_) => CommandResult::Continue(Some("true".to_string())),
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
