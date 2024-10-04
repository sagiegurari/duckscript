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
        pckg::concat(&self.package, "WriteText")
    }

    fn aliases(&self) -> Vec<String> {
        vec!["writefile".to_string(), "write_text_file".to_string()]
    }

    fn help(&self) -> String {
        include_str!("help.md").to_string()
    }

    fn clone_and_box(&self) -> Box<dyn Command> {
        Box::new((*self).clone())
    }

    fn run(&self, context: CommandInvocationContext) -> CommandResult {
        if context.arguments.is_empty() {
            CommandResult::Error("File name and text not provided.".to_string())
        } else if context.arguments.len() == 1 {
            CommandResult::Error("Text not provided.".to_string())
        } else {
            let result = io::write_text_file(&context.arguments[0], &context.arguments[1]);

            match result {
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
