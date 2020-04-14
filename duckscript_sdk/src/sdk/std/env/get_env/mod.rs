use crate::utils::pckg;
use duckscript::types::command::{Command, CommandResult};
use std::env;

#[cfg(test)]
#[path = "./mod_test.rs"]
mod mod_test;

#[derive(Clone)]
pub(crate) struct CommandImpl {
    package: String,
}

impl Command for CommandImpl {
    fn name(&self) -> String {
        pckg::concat(&self.package, "GetVar")
    }

    fn aliases(&self) -> Vec<String> {
        vec!["get_env".to_string()]
    }

    fn help(&self) -> String {
        include_str!("help.md").to_string()
    }

    fn clone_and_box(&self) -> Box<dyn Command> {
        Box::new((*self).clone())
    }

    fn run(&self, arguments: Vec<String>) -> CommandResult {
        if arguments.is_empty() {
            CommandResult::Error("Missing environment variable name.".to_string())
        } else {
            match env::var(&arguments[0]) {
                Ok(value) => CommandResult::Continue(Some(value)),
                Err(_) => CommandResult::Continue(None),
            }
        }
    }
}

pub(crate) fn create(package: &str) -> Box<dyn Command> {
    Box::new(CommandImpl {
        package: package.to_string(),
    })
}
