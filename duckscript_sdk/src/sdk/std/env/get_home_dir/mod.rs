use crate::utils::pckg;
use duckscript::types::command::{Command, CommandResult};
use home;

#[cfg(test)]
#[path = "./mod_test.rs"]
mod mod_test;

#[derive(Clone)]
pub(crate) struct CommandImpl {
    package: String,
}

impl Command for CommandImpl {
    fn name(&self) -> String {
        pckg::concat(&self.package, "GetHomeDirectory")
    }

    fn aliases(&self) -> Vec<String> {
        vec!["get_home_dir".to_string()]
    }

    fn help(&self) -> String {
        include_str!("help.md").to_string()
    }

    fn clone_and_box(&self) -> Box<dyn Command> {
        Box::new((*self).clone())
    }

    fn run(&self, _arguments: Vec<String>) -> CommandResult {
        match home::home_dir() {
            Some(directory) => {
                let directory_str = directory.to_string_lossy().into_owned();
                CommandResult::Continue(Some(directory_str))
            }
            None => CommandResult::Error("Unable to find user home directory.".to_string()),
        }
    }
}

pub(crate) fn create(package: &str) -> Box<dyn Command> {
    Box::new(CommandImpl {
        package: package.to_string(),
    })
}
