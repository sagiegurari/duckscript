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
        pckg::concat(&self.package, "IsPathNewer")
    }

    fn aliases(&self) -> Vec<String> {
        vec!["is_path_newer".to_string()]
    }

    fn help(&self) -> String {
        include_str!("help.md").to_string()
    }

    fn clone_and_box(&self) -> Box<dyn Command> {
        Box::new((*self).clone())
    }

    fn run(&self, context: CommandInvocationContext) -> CommandResult {
        if context.arguments.len() < 2 {
            CommandResult::Error("Paths not provided.".to_string())
        } else {
            let newer = match io::get_last_modified_time(&context.arguments[0]) {
                Ok(time) => time,
                Err(error) => return CommandResult::Error(error),
            };

            let older = match io::get_last_modified_time(&context.arguments[1]) {
                Ok(time) => time,
                Err(error) => return CommandResult::Error(error),
            };

            let result = if newer > older { true } else { false };
            CommandResult::Continue(Some(result.to_string()))
        }
    }
}

pub(crate) fn create(package: &str) -> Box<dyn Command> {
    Box::new(CommandImpl {
        package: package.to_string(),
    })
}
