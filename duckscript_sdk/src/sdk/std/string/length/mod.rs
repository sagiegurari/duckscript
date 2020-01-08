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
        pckg::concat(&self.package, "Length")
    }

    fn aliases(&self) -> Vec<String> {
        vec!["length".to_string(), "strlen".to_string()]
    }

    fn help(&self) -> String {
        include_str!("help.md").to_string()
    }

    fn run(&self, arguments: Vec<String>) -> CommandResult {
        if arguments.is_empty() {
            CommandResult::Error("No argument provided.".to_string())
        } else {
            let string_len = arguments[0].len();

            CommandResult::Continue(Some(string_len.to_string()))
        }
    }
}

pub(crate) fn create(package: &str) -> Box<dyn Command> {
    Box::new(CommandImpl {
        package: package.to_string(),
    })
}
