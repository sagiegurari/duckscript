use crate::utils::pckg;
use duckscript::types::command::{Command, CommandResult};
use std::env;

#[cfg(test)]
#[path = "./mod_test.rs"]
mod mod_test;

struct CommandImpl {
    package: String,
}

impl Command for CommandImpl {
    fn name(&self) -> String {
        pckg::concat(&self.package, "SetVar")
    }

    fn aliases(&self) -> Vec<String> {
        vec!["set_env".to_string()]
    }

    fn help(&self) -> String {
        include_str!("help.md").to_string()
    }

    fn run(&self, arguments: Vec<String>) -> CommandResult {
        if arguments.is_empty() {
            CommandResult::Error("Missing environment variable name and value.".to_string())
        } else if arguments.len() == 1 {
            CommandResult::Error("Missing environment variable value.".to_string())
        } else {
            env::set_var(&arguments[0], &arguments[1]);

            CommandResult::Continue(Some("true".to_string()))
        }
    }
}

pub(crate) fn create(package: &str) -> Box<dyn Command> {
    Box::new(CommandImpl {
        package: package.to_string(),
    })
}
