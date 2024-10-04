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
        pckg::concat(&self.package, "SetByName")
    }

    fn aliases(&self) -> Vec<String> {
        vec!["set_by_name".to_string()]
    }

    fn help(&self) -> String {
        include_str!("help.md").to_string()
    }

    fn clone_and_box(&self) -> Box<dyn Command> {
        Box::new((*self).clone())
    }

    fn run(&self, context: CommandInvocationContext) -> CommandResult {
        if context.arguments.is_empty() {
            CommandResult::Error("Missing variable name.".to_string())
        } else {
            let output = if context.arguments.len() > 1 {
                context
                    .variables
                    .insert(context.arguments[0].clone(), context.arguments[1].clone());
                Some(context.arguments[1].clone())
            } else {
                context.variables.remove(&context.arguments[0]);
                None
            };

            CommandResult::Continue(output)
        }
    }
}

pub(crate) fn create(package: &str) -> Box<dyn Command> {
    Box::new(CommandImpl {
        package: package.to_string(),
    })
}
