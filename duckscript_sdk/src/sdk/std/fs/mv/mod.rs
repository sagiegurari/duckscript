use crate::utils::io::ends_with_separator;
use crate::utils::pckg;
use duckscript::types::command::{Command, CommandInvocationContext, CommandResult};
use fs_extra::{dir, move_items};
use fsio::directory::create_parent;
use std::fs::rename;
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

    fn run(&self, context: CommandInvocationContext) -> CommandResult {
        if context.arguments.len() < 2 {
            CommandResult::Error("Paths not provided.".to_string())
        } else {
            let source_path = Path::new(&context.arguments[0]);

            if !source_path.exists() {
                CommandResult::Error("Path does not exist.".to_string())
            } else {
                let target_path = Path::new(&context.arguments[1]);
                let source_ends_with_separator = ends_with_separator(&context.arguments[0]);
                let source_file = source_path.is_file();
                let target_exists = target_path.exists();
                let target_ends_with_separator = ends_with_separator(&context.arguments[1]);
                let target_file = if target_exists {
                    target_path.is_file()
                } else {
                    !target_ends_with_separator && target_path.extension().is_some()
                };

                if source_file && target_file {
                    match create_parent(&target_path) {
                        Ok(_) => {
                            let options = fs_extra::file::CopyOptions::new().overwrite(true);
                            match fs_extra::file::move_file(source_path, &target_path, &options) {
                                Ok(_) => CommandResult::Continue(Some("true".to_string())),
                                Err(error) => CommandResult::Error(error.to_string()),
                            }
                        }
                        Err(error) => CommandResult::Error(error.to_string()),
                    }
                } else if !source_file
                    && !target_file
                    && !source_ends_with_separator
                    && !target_ends_with_separator
                {
                    // rename directory
                    match rename(source_path, target_path) {
                        Ok(_) => CommandResult::Continue(Some("true".to_string())),
                        Err(error) => CommandResult::Error(error.to_string()),
                    }
                } else {
                    match fsio::directory::create(&context.arguments[1]) {
                        Ok(_) => {
                            let options = dir::CopyOptions::new();
                            let from_paths = vec![&context.arguments[0]];
                            match move_items(&from_paths, &context.arguments[1], &options) {
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
