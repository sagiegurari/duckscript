use crate::sdk::std::on_error::{get_value, EXIT_ON_ERROR_KEY, STATE_KEY};
use crate::utils::state::get_core_sub_state_for_command;
use crate::utils::{condition, pckg};
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
        pckg::concat(&self.package, "OnError")
    }

    fn aliases(&self) -> Vec<String> {
        vec!["on_error".to_string()]
    }

    fn help(&self) -> String {
        "".to_string()
    }

    fn clone_and_box(&self) -> Box<dyn Command> {
        Box::new((*self).clone())
    }

    fn run(&self, context: CommandInvocationContext) -> CommandResult {
        if !context.arguments.is_empty() {
            let error = context.arguments[0].clone();

            let exit_on_error = get_value(context.state, EXIT_ON_ERROR_KEY.to_string());
            let should_crash = condition::is_true(exit_on_error);

            if should_crash {
                CommandResult::Crash(error)
            } else {
                let (line, source) = if context.arguments.len() > 1 {
                    let line = context.arguments[1].clone();
                    let source = if context.arguments.len() > 2 {
                        context.arguments[2].clone()
                    } else {
                        "".to_string()
                    };

                    (line, source)
                } else {
                    ("".to_string(), "".to_string())
                };

                let sub_state =
                    get_core_sub_state_for_command(context.state, STATE_KEY.to_string());
                sub_state.insert("error".to_string(), StateValue::String(error));
                sub_state.insert("line".to_string(), StateValue::String(line));
                sub_state.insert("source".to_string(), StateValue::String(source));

                CommandResult::Continue(Some("false".to_string()))
            }
        } else {
            CommandResult::Crash("Invalid input provided.".to_string())
        }
    }
}

pub(crate) fn create(package: &str) -> Box<dyn Command> {
    Box::new(CommandImpl {
        package: package.to_string(),
    })
}
