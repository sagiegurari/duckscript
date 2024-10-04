use crate::utils::pckg;
use duckscript::types::command::{Command, CommandInvocationContext, CommandResult};

#[cfg(test)]
#[path = "./mod_test.rs"]
mod mod_test;

#[derive(Clone)]
pub(crate) struct CommandImpl {
    package: String,
}

impl Command for CommandImpl {
    fn name(&self) -> String {
        pckg::concat(&self.package, "AssertEquals")
    }

    fn aliases(&self) -> Vec<String> {
        vec!["assert_eq".to_string()]
    }

    fn help(&self) -> String {
        include_str!("help.md").to_string()
    }

    fn clone_and_box(&self) -> Box<dyn Command> {
        Box::new((*self).clone())
    }

    fn run(&self, context: CommandInvocationContext) -> CommandResult {
        if context.arguments.len() < 2 {
            CommandResult::Crash("Assert failed, two arguments are required.".to_string())
        } else {
            let passed = context.arguments[0] == context.arguments[1];

            if passed {
                CommandResult::Continue(Some("true".to_string()))
            } else {
                let error_message = if context.arguments.len() == 2 {
                    format!(
                        "Assert failed, value: {} does not match: {}",
                        &context.arguments[0], &context.arguments[1]
                    )
                    .to_string()
                } else {
                    context.arguments[2].clone()
                };

                CommandResult::Crash(error_message)
            }
        }
    }
}

pub(crate) fn create(package: &str) -> Box<dyn Command> {
    Box::new(CommandImpl {
        package: package.to_string(),
    })
}
