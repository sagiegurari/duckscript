use crate::utils::pckg;
use duckscript::types::command::{Command, CommandResult};

#[cfg(test)]
#[path = "./mod_test.rs"]
mod mod_test;

struct CommandImpl {
    package: String,
}

impl Command for CommandImpl {
    fn name(&self) -> String {
        pckg::concat(&self.package, "Exit")
    }

    fn aliases(&self) -> Vec<String> {
        vec!["exit".to_string()]
    }

    fn help(&self) -> String {
        include_str!("help.md").to_string()
    }

    fn run(&self, arguments: Vec<String>) -> CommandResult {
        if arguments.is_empty() {
            CommandResult::Exit(Some("0".to_string()))
        } else {
            match arguments[0].parse::<u32>() {
                Ok(code) => CommandResult::Exit(Some(arguments[0].clone())),
                Err(_) => CommandResult::Error(format!("Invalid exit code: {}", arguments[0]).to_string()),
            }
        }
    }
}

pub(crate) fn create(package: &str) -> Box<dyn Command> {
    Box::new(CommandImpl {
        package: package.to_string(),
    })
}
