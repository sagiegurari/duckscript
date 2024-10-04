use crate::utils::pckg;
use crate::utils::state::get_handles_sub_state;
use base64::Engine;
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
        pckg::concat(&self.package, "Base64Encode")
    }

    fn aliases(&self) -> Vec<String> {
        vec!["base64_encode".to_string()]
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

            let key = &context.arguments[0];

            match state.get(key) {
                Some(state_value) => match state_value {
                    StateValue::ByteArray(binary) => {
                        let encoded = base64::engine::general_purpose::STANDARD.encode(binary);

                        CommandResult::Continue(Some(encoded))
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
