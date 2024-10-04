use crate::utils::pckg;
use duckscript::types::command::{Command, CommandInvocationContext, CommandResult, GoToValue};

#[cfg(test)]
#[path = "./mod_test.rs"]
mod mod_test;

#[derive(Clone)]
pub(crate) struct CommandImpl {
    package: String,
}

impl Command for CommandImpl {
    fn name(&self) -> String {
        pckg::concat(&self.package, "GoTo")
    }

    fn aliases(&self) -> Vec<String> {
        vec!["goto".to_string()]
    }

    fn help(&self) -> String {
        include_str!("help.md").to_string()
    }

    fn clone_and_box(&self) -> Box<dyn Command> {
        Box::new((*self).clone())
    }

    fn run(&self, context: CommandInvocationContext) -> CommandResult {
        if context.arguments.is_empty() {
            CommandResult::Error("Label not provided.".to_string())
        } else if context.arguments.len() > 1 {
            CommandResult::Error("Multiple labels provided.".to_string())
        } else {
            let label = context.arguments[0].clone();

            if label.starts_with(":") {
                CommandResult::GoTo(None, GoToValue::Label(label))
            } else {
                CommandResult::Error(format!("Invalid label: {} provided.", &label).to_string())
            }
        }
    }
}

pub(crate) fn create(package: &str) -> Box<dyn Command> {
    Box::new(CommandImpl {
        package: package.to_string(),
    })
}
