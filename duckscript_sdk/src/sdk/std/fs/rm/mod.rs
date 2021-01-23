use crate::utils::{flags, pckg};
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
        pckg::concat(&self.package, "DeletePath")
    }

    fn aliases(&self) -> Vec<String> {
        vec!["rm".to_string()]
    }

    fn help(&self) -> String {
        include_str!("help.md").to_string()
    }

    fn clone_and_box(&self) -> Box<dyn Command> {
        Box::new((*self).clone())
    }

    fn run(&self, arguments: Vec<String>) -> CommandResult {
        if arguments.is_empty()
            || (arguments.len() == 1 && flags::is_unix_flags_argument(&arguments[0]))
        {
            CommandResult::Error("Path not provided.".to_string())
        } else {
            let (start_index, recursive) = if arguments.len() == 1 {
                (0, false)
            } else if flags::is_unix_flags_argument(&arguments[0]) {
                let recursive = flags::is_unix_flag_exists('r', &arguments[0]);
                (1, recursive)
            } else {
                (0, false)
            };

            let end_index = arguments.len();
            for index in start_index..end_index {
                let path = Path::new(&arguments[index]);

                let result = if !path.exists() {
                    Ok(())
                } else if path.is_file() {
                    fs::remove_file(&path)
                } else if recursive {
                    fs::remove_dir_all(&path)
                } else {
                    fs::remove_dir(&path)
                };

                if let Err(error) = result {
                    return CommandResult::Error(error.to_string());
                }
            }

            CommandResult::Continue(Some("true".to_string()))
        }
    }
}

pub(crate) fn create(package: &str) -> Box<dyn Command> {
    Box::new(CommandImpl {
        package: package.to_string(),
    })
}
