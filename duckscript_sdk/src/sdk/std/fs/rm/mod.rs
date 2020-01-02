use crate::utils::{flags, pckg};
use duckscript::types::command::{Command, CommandResult};
use std::fs;
use std::path::Path;

#[cfg(test)]
#[path = "./mod_test.rs"]
mod mod_test;

struct CommandImpl {
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

    fn run(&self, arguments: Vec<String>) -> CommandResult {
        if arguments.is_empty() {
            CommandResult::Error("Path not provided.".to_string())
        } else {
            let (path_str, recursive) = if arguments.len() == 1 {
                (&arguments[0], false)
            } else if flags::is_unix_flags_argument(&arguments[0]) {
                let recursive = flags::is_unix_flag_exists('r', &arguments[0]);
                (&arguments[1], recursive)
            } else {
                (&arguments[0], false)
            };

            let path = Path::new(path_str);

            let result = if !path.exists() {
                Ok(())
            } else if path.is_file() {
                fs::remove_file(&arguments[0])
            } else if recursive {
                fs::remove_dir_all(&path)
            } else {
                fs::remove_dir(&path)
            };

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
