use crate::utils::pckg;
use duckscript::types::command::{Command, CommandInvocationContext, CommandResult};
use fsio::path::as_path::AsPath;

#[cfg(test)]
#[path = "./mod_test.rs"]
mod mod_test;

#[derive(Clone)]
pub(crate) struct CommandImpl {
    package: String,
}

impl Command for CommandImpl {
    fn name(&self) -> String {
        pckg::concat(&self.package, "IsDirectory")
    }

    fn aliases(&self) -> Vec<String> {
        vec!["is_directory".to_string(), "is_dir".to_string()]
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
            let path = &context.arguments[0].as_path();
            CommandResult::Continue(Some(path.is_dir().to_string()))
        }
    }
}

pub(crate) fn create(package: &str) -> Box<dyn Command> {
    Box::new(CommandImpl {
        package: package.to_string(),
    })
}
