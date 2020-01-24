use crate::utils::pckg;
use duckscript::types::command::{Command, CommandResult};
use std::fs;
use std::path::Path;

#[cfg(test)]
#[path = "./mod_test.rs"]
mod mod_test;

#[derive(Clone)]
pub(crate) struct CommandImpl {
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

    fn clone_and_box(&self) -> Box<dyn Command> {
        Box::new((*self).clone())
    }

    fn run(&self, arguments: Vec<String>) -> CommandResult {
        if arguments.is_empty() {
            CommandResult::Error("Directory path not provided.".to_string())
        } else {
            let path = Path::new(&arguments[0]);
            if !path.exists() {
                CommandResult::Continue(Some("true".to_string()))
            } else {
                let result = fs::remove_dir(&arguments[0]);

                match result {
                    Ok(_) => CommandResult::Continue(Some("true".to_string())),
                    Err(_) => CommandResult::Continue(Some("false".to_string())),
                }
            }
        }
    }
}

pub(crate) fn create(package: &str) -> Box<dyn Command> {
    Box::new(CommandImpl {
        package: package.to_string(),
    })
}
