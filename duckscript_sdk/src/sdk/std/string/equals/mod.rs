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
        pckg::concat(&self.package, "Equals")
    }

    fn aliases(&self) -> Vec<String> {
        vec!["equals".to_string(), "eq".to_string()]
    }

    fn help(&self) -> String {
        include_str!("help.md").to_string()
    }

    fn run(&self, arguments: Vec<String>) -> CommandResult {
        if arguments.len() < 2 {
            CommandResult::Error("Two arguments are required.".to_string())
        } else {
            let result = arguments[0] == arguments[1];

            CommandResult::Continue(Some(result.to_string()))
        }
    }
}

pub(crate) fn create(package: &str) -> Box<dyn Command> {
    Box::new(CommandImpl {
        package: package.to_string(),
    })
}
