use crate::utils::pckg;
use duckscript::types::command::{Command, CommandResult};
use fs_extra::{dir, move_items};
use fsio;
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
        pckg::concat(&self.package, "MovePath")
    }

    fn aliases(&self) -> Vec<String> {
        vec!["mv".to_string()]
    }

    fn help(&self) -> String {
        include_str!("help.md").to_string()
    }

    fn clone_and_box(&self) -> Box<dyn Command> {
        Box::new((*self).clone())
    }

    fn run(&self, arguments: Vec<String>) -> CommandResult {
        if arguments.len() < 2 {
            CommandResult::Error("Paths not provided.".to_string())
        } else {
            let source_path = Path::new(&arguments[0]);

            if !source_path.exists() {
                CommandResult::Error("Path does not exist.".to_string())
            } else {
                let target_path = Path::new(&arguments[1]);
                let source_file = source_path.is_file();
                let target_file = target_path.is_file();

                if source_file && target_file {
                    match fs::rename(&arguments[0], &arguments[1]) {
                        Ok(_) => CommandResult::Continue(Some("true".to_string())),
                        Err(error) => CommandResult::Error(error.to_string()),
                    }
                } else {
                    match fsio::directory::create(&arguments[1]) {
                        Ok(_) => {
                            let options = dir::CopyOptions::new();
                            let from_paths = vec![&arguments[0]];
                            match move_items(&from_paths, &arguments[1], &options) {
                                Ok(_) => CommandResult::Continue(Some("true".to_string())),
                                Err(error) => CommandResult::Error(error.to_string()),
                            }
                        }
                        Err(error) => CommandResult::Error(error.to_string()),
                    }
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
