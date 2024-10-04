use crate::utils::pckg;
use crate::utils::state::{get_handles_sub_state, mutate_list};
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
        pckg::concat(&self.package, "ArrayPush")
    }

    fn aliases(&self) -> Vec<String> {
        vec![
            "array_push".to_string(),
            "array_add".to_string(),
            "array_put".to_string(),
        ]
    }

    fn help(&self) -> String {
        include_str!("help.md").to_string()
    }

    fn clone_and_box(&self) -> Box<dyn Command> {
        Box::new((*self).clone())
    }

    fn run(&self, context: CommandInvocationContext) -> CommandResult {
        if context.arguments.is_empty() {
            CommandResult::Error("Array handle not provided.".to_string())
        } else {
            let state = get_handles_sub_state(context.state);

            let key = context.arguments[0].clone();

            let result = mutate_list(key, state, |list| {
                for argument in &context.arguments[1..] {
                    list.push(StateValue::String(argument.to_string()))
                }

                Ok(None)
            });

            match result {
                Ok(_) => CommandResult::Continue(Some("true".to_string())),
                Err(error) => CommandResult::Error(error),
            }
        }
    }
}

pub(crate) fn create(package: &str) -> Box<dyn Command> {
    Box::new(CommandImpl {
        package: package.to_string(),
    })
}
