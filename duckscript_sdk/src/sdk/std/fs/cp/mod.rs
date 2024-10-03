use crate::utils::pckg;
use duckscript::types::command::{Command, CommandArgs, CommandResult};
use fs_extra::dir;
use fsio::directory::create_parent;
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
        pckg::concat(&self.package, "CopyPath")
    }

    fn aliases(&self) -> Vec<String> {
        vec!["cp".to_string()]
    }

    fn help(&self) -> String {
        include_str!("help.md").to_string()
    }

    fn clone_and_box(&self) -> Box<dyn Command> {
        Box::new((*self).clone())
    }

    fn run(&self, arguments: CommandArgs) -> CommandResult {
        if arguments.args.len() < 2 {
            CommandResult::Error("Paths not provided.".to_string())
        } else {
            let source_path_str = &arguments.args[0];
            let source_path = Path::new(source_path_str);
            if !source_path.exists() {
                CommandResult::Error("Path does not exist.".to_string())
            } else {
                let source_file = source_path.is_file();

                let target_path_str = &arguments.args[1];

                if source_file {
                    match create_parent(target_path_str) {
                        Ok(_) => match fs::copy(source_path_str, target_path_str) {
                            Ok(_) => CommandResult::Continue(Some("true".to_string())),
                            Err(error) => CommandResult::Error(error.to_string()),
                        },
                        Err(error) => CommandResult::Error(error.to_string()),
                    }
                } else {
                    match fsio::directory::create(target_path_str) {
                        Ok(_) => {
                            let options = dir::CopyOptions::new();

                            match dir::copy(source_path_str, target_path_str, &options) {
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
