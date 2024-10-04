use crate::utils::{condition, pckg};
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
        pckg::concat(&self.package, "AssertFalse")
    }

    fn aliases(&self) -> Vec<String> {
        vec!["assert_false".to_string()]
    }

    fn help(&self) -> String {
        include_str!("help.md").to_string()
    }

    fn clone_and_box(&self) -> Box<dyn Command> {
        Box::new((*self).clone())
    }

    fn run(&self, context: CommandInvocationContext) -> CommandResult {
        if context.arguments.is_empty() {
            CommandResult::Crash("Assert failed, empty value.".to_string())
        } else {
            let passed = condition::is_true(Some(context.arguments[0].clone()));

            if passed {
                let error_message = if context.arguments.len() == 1 {
                    format!("Assert failed, value is true: {}", &context.arguments[0]).to_string()
                } else {
                    context.arguments[1].clone()
                };

                CommandResult::Crash(error_message)
            } else {
                CommandResult::Continue(Some("true".to_string()))
            }
        }
    }
}

pub(crate) fn create(package: &str) -> Box<dyn Command> {
    Box::new(CommandImpl {
        package: package.to_string(),
    })
}
