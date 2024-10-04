use crate::utils::state::get_handles_sub_state;
use crate::utils::{io, pckg};
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
        pckg::concat(&self.package, "WriteBytes")
    }

    fn aliases(&self) -> Vec<String> {
        vec!["writebinfile".to_string(), "write_binary_file".to_string()]
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
            CommandResult::Error("Binary data handle not provided.".to_string())
        } else {
            let state = get_handles_sub_state(context.state);

            let key = &context.arguments[1];

            match state.get(key) {
                Some(state_value) => match state_value {
                    StateValue::ByteArray(binary) => {
                        let result = io::write_to_file(&context.arguments[0], &binary, false);

                        match result {
                            Ok(_) => CommandResult::Continue(Some("true".to_string())),
                            Err(_) => CommandResult::Continue(Some("false".to_string())),
                        }
                    }
                    _ => CommandResult::Error("Invalid handle provided.".to_string()),
                },
                None => CommandResult::Error(
                    format!("Array for handle: {} not found.", key).to_string(),
                ),
            }
        }
    }
}

pub(crate) fn create(package: &str) -> Box<dyn Command> {
    Box::new(CommandImpl {
        package: package.to_string(),
    })
}
