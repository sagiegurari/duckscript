use crate::utils::pckg;
use duckscript::types::command::{Command, CommandResult};
use std::fs;
use std::time::SystemTime;

#[cfg(test)]
#[path = "./mod_test.rs"]
mod mod_test;

#[derive(Clone)]
pub(crate) struct CommandImpl {
    package: String,
}

impl Command for CommandImpl {
    fn name(&self) -> String {
        pckg::concat(&self.package, "GetLastModifiedTime")
    }

    fn aliases(&self) -> Vec<String> {
        vec!["get_last_modified_time".to_string()]
    }

    fn help(&self) -> String {
        include_str!("help.md").to_string()
    }

    fn clone_and_box(&self) -> Box<dyn Command> {
        Box::new((*self).clone())
    }

    fn run(&self, arguments: Vec<String>) -> CommandResult {
        if arguments.is_empty() {
            CommandResult::Error("Path not provided.".to_string())
        } else {
            match fs::metadata(&arguments[0]) {
                Ok(metadata) => match metadata.modified() {
                    Ok(time) => match time.duration_since(SystemTime::UNIX_EPOCH) {
                        Ok(duration) => {
                            CommandResult::Continue(Some(duration.as_millis().to_string()))
                        }
                        Err(error) => CommandResult::Error(error.to_string()),
                    },
                    Err(error) => CommandResult::Error(error.to_string()),
                },
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
