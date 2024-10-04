use crate::utils::pckg;
use crate::utils::state::put_handle;
use duckscript::types::command::{Command, CommandInvocationContext, CommandResult};
use duckscript::types::runtime::StateValue;

#[cfg(test)]
#[path = "./mod_test.rs"]
mod mod_test;

#[derive(Clone)]
pub(crate) struct CommandImpl {
    package: String,
}

impl Command for CommandImpl {
    fn name(&self) -> String {
        pckg::concat(&self.package, "Split")
    }

    fn aliases(&self) -> Vec<String> {
        vec!["split".to_string()]
    }

    fn help(&self) -> String {
        include_str!("help.md").to_string()
    }

    fn clone_and_box(&self) -> Box<dyn Command> {
        Box::new((*self).clone())
    }

    fn run(&self, context: CommandInvocationContext) -> CommandResult {
        if context.arguments.len() < 2 {
            CommandResult::Error("Invalid input provided.".to_string())
        } else {
            let split = context.arguments[0].split(&context.arguments[1]);
            let values = split.collect::<Vec<&str>>();

            let mut array = vec![];
            for value in values {
                array.push(StateValue::String(value.to_string()));
            }

            let key = put_handle(context.state, StateValue::List(array));

            CommandResult::Continue(Some(key))
        }
    }
}

pub(crate) fn create(package: &str) -> Box<dyn Command> {
    Box::new(CommandImpl {
        package: package.to_string(),
    })
}
