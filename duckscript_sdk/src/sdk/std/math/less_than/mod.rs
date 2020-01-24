use crate::utils::pckg;
use duckscript::types::command::{Command, CommandResult};

#[cfg(test)]
#[path = "./mod_test.rs"]
mod mod_test;

#[derive(Clone)]
pub(crate) struct CommandImpl {
    package: String,
}

impl Command for CommandImpl {
    fn name(&self) -> String {
        pckg::concat(&self.package, "LessThan")
    }

    fn aliases(&self) -> Vec<String> {
        vec!["less_than".to_string()]
    }

    fn help(&self) -> String {
        include_str!("help.md").to_string()
    }

    fn clone_and_box(&self) -> Box<dyn Command> {
        Box::new((*self).clone())
    }

    fn run(&self, arguments: Vec<String>) -> CommandResult {
        if arguments.len() != 2 {
            CommandResult::Error("Invalid/Missing input.".to_string())
        } else {
            let left: f64 = match arguments[0].parse() {
                Ok(value) => value,
                Err(_) => {
                    return CommandResult::Error(
                        format!("Non numeric value: {} provided.", &arguments[0]).to_string(),
                    );
                }
            };
            let right: f64 = match arguments[1].parse() {
                Ok(value) => value,
                Err(_) => {
                    return CommandResult::Error(
                        format!("Non numeric value: {} provided.", &arguments[1]).to_string(),
                    );
                }
            };

            let result = if left < right { true } else { false };

            CommandResult::Continue(Some(result.to_string()))
        }
    }
}

pub(crate) fn create(package: &str) -> Box<dyn Command> {
    Box::new(CommandImpl {
        package: package.to_string(),
    })
}
