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
        pckg::concat(&self.package, "Echo")
    }

    fn aliases(&self) -> Vec<String> {
        vec!["echo".to_string()]
    }

    fn help(&self) -> String {
        include_str!("help.md").to_string()
    }

    fn clone_and_box(&self) -> Box<dyn Command> {
        Box::new((*self).clone())
    }

    fn run(&self, context: CommandInvocationContext) -> CommandResult {
        for argument in &context.arguments {
            if let Err(error) = write!(context.env.out, "{} ", argument) {
                return CommandResult::Error(error.to_string());
            }
        }

        match writeln!(context.env.out, "") {
            Ok(_) => CommandResult::Continue(Some(context.arguments.len().to_string())),
            Err(error) => CommandResult::Error(error.to_string()),
        }
    }
}

pub(crate) fn create(package: &str) -> Box<dyn Command> {
    Box::new(CommandImpl {
        package: package.to_string(),
    })
}
