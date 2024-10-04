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
        pckg::concat(&self.package, "GetByName")
    }

    fn aliases(&self) -> Vec<String> {
        vec!["get_by_name".to_string()]
    }

    fn help(&self) -> String {
        include_str!("help.md").to_string()
    }

    fn clone_and_box(&self) -> Box<dyn Command> {
        Box::new((*self).clone())
    }

    fn run(&self, context: CommandInvocationContext) -> CommandResult {
        let output = if context.arguments.is_empty() {
            None
        } else {
            match context.variables.get(&context.arguments[0]) {
                Some(ref value) => Some(value.to_string()),
                None => None,
            }
        };

        CommandResult::Continue(output)
    }
}

pub(crate) fn create(package: &str) -> Box<dyn Command> {
    Box::new(CommandImpl {
        package: package.to_string(),
    })
}
