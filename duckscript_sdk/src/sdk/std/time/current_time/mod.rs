use crate::utils::pckg;
use duckscript::types::command::{Command, CommandResult};
use std::time::{SystemTime, UNIX_EPOCH};

#[cfg(test)]
#[path = "./mod_test.rs"]
mod mod_test;

#[derive(Clone)]
pub(crate) struct CommandImpl {
    package: String,
}

impl Command for CommandImpl {
    fn name(&self) -> String {
        pckg::concat(&self.package, "CurrentTimeMillies")
    }

    fn aliases(&self) -> Vec<String> {
        vec!["current_time".to_string()]
    }

    fn help(&self) -> String {
        include_str!("help.md").to_string()
    }

    fn clone_and_box(&self) -> Box<dyn Command> {
        Box::new((*self).clone())
    }

    fn run(&self, _arguments: Vec<String>) -> CommandResult {
        match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(value) => CommandResult::Continue(Some(value.as_millis().to_string())),
            Err(error) => CommandResult::Error(error.to_string()),
        }
    }
}

pub(crate) fn create(package: &str) -> Box<dyn Command> {
    Box::new(CommandImpl {
        package: package.to_string(),
    })
}
