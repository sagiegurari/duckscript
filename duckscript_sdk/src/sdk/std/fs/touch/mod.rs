use crate::utils::{io, pckg};
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
        pckg::concat(&self.package, "CreateEmptyFile")
    }

    fn aliases(&self) -> Vec<String> {
        vec!["touch".to_string()]
    }

    fn help(&self) -> String {
        include_str!("help.md").to_string()
    }

    fn clone_and_box(&self) -> Box<dyn Command> {
        Box::new((*self).clone())
    }

    fn run(&self, context: CommandInvocationContext) -> CommandResult {
        if context.arguments.is_empty() {
            CommandResult::Error("Path not provided.".to_string())
        } else {
            match io::create_empty_file(&context.arguments[0]) {
                Ok(_) => CommandResult::Continue(Some("true".to_string())),
                Err(_) => CommandResult::Continue(Some("false".to_string())),
            }
        }
    }
}

pub(crate) fn create(package: &str) -> Box<dyn Command> {
    Box::new(CommandImpl {
        package: package.to_string(),
    })
}
