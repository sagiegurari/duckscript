use crate::utils::{condition, pckg};
use duckscript::types::command::{Command, CommandResult};

#[cfg(test)]
#[path = "./mod_test.rs"]
mod mod_test;

struct CommandImpl {
    package: String,
}

impl Command for CommandImpl {
    fn name(&self) -> String {
        pckg::concat(&self.package, "Assert")
    }

    fn aliases(&self) -> Vec<String> {
        vec!["assert".to_string()]
    }

    fn help(&self) -> String {
        include_str!("help.md").to_string()
    }

    fn run(&self, arguments: Vec<String>) -> CommandResult {
        if arguments.is_empty() {
            CommandResult::Error("Assert failed, empty value.".to_string())
        } else {
            let passed = condition::is_true(Some(arguments[0].clone()));

            if passed {
                CommandResult::Continue(Some("true".to_string()))
            } else {
                let error_message = if arguments.len() == 1 {
                    format!("Assert failed, value is falsy: {}", &arguments[0]).to_string()
                } else {
                    arguments[1].clone()
                };

                CommandResult::Error(error_message)
            }
        }
    }
}

pub(crate) fn create(package: &str) -> Box<dyn Command> {
    Box::new(CommandImpl {
        package: package.to_string(),
    })
}
