use crate::utils::pckg;
use duckscript::types::command::{Command, CommandResult};
use home;
use std::env;
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
        pckg::concat(&self.package, "SetCurrentDirectory")
    }

    fn aliases(&self) -> Vec<String> {
        vec![
            "cd".to_string(),
            "set_current_dir".to_string(),
            "set_current_directory".to_string(),
        ]
    }

    fn help(&self) -> String {
        include_str!("help.md").to_string()
    }

    fn clone_and_box(&self) -> Box<dyn Command> {
        Box::new((*self).clone())
    }

    fn run(&self, arguments: Vec<String>) -> CommandResult {
        let directory_option = if arguments.is_empty() {
            home::home_dir()
        } else {
            let path = Path::new(&arguments[0]);
            Some(path.to_path_buf())
        };

        match directory_option {
            Some(directory) => match env::set_current_dir(&directory) {
                Ok(_) => {
                    let directory_str = directory.to_string_lossy().into_owned();
                    CommandResult::Continue(Some(directory_str))
                }
                Err(_) => CommandResult::Continue(None),
            },
            None => CommandResult::Continue(None),
        }
    }
}

pub(crate) fn create(package: &str) -> Box<dyn Command> {
    Box::new(CommandImpl {
        package: package.to_string(),
    })
}
