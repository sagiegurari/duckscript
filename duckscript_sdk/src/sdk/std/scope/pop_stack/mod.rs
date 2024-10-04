use crate::utils::{pckg, scope};
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
        pckg::concat(&self.package, "PopStack")
    }

    fn aliases(&self) -> Vec<String> {
        vec!["scope_pop_stack".to_string()]
    }

    fn help(&self) -> String {
        include_str!("help.md").to_string()
    }

    fn clone_and_box(&self) -> Box<dyn Command> {
        Box::new((*self).clone())
    }

    fn run(&self, context: CommandInvocationContext) -> CommandResult {
        let copy = if context.arguments.is_empty() {
            &[]
        } else if context.arguments[0] == "--copy" {
            &context.arguments[1..]
        } else {
            &[]
        };

        match scope::pop(context.variables, context.state, &copy) {
            Ok(_) => CommandResult::Continue(Some("true".to_string())),
            Err(error) => CommandResult::Error(error),
        }
    }
}

pub(crate) fn create(package: &str) -> Box<dyn Command> {
    Box::new(CommandImpl {
        package: package.to_string(),
    })
}
