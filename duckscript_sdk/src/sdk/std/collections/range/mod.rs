use crate::utils::pckg;
use crate::utils::state::put_handle;
use duckscript::types::command::{Command, CommandArgs, CommandResult};
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
        pckg::concat(&self.package, "Range")
    }

    fn aliases(&self) -> Vec<String> {
        vec!["range".to_string()]
    }

    fn help(&self) -> String {
        include_str!("help.md").to_string()
    }

    fn clone_and_box(&self) -> Box<dyn Command> {
        Box::new((*self).clone())
    }

    fn run(&self, arguments: CommandArgs) -> CommandResult {
        if arguments.args.len() < 2 {
            CommandResult::Error("Invalid arguments provided.".to_string())
        } else {
            let start: i64 = match arguments.args[0].parse() {
                Ok(value) => value,
                Err(_) => {
                    return CommandResult::Error(
                        format!("Non numeric value: {} provided.", &arguments.args[0]).to_string(),
                    );
                }
            };

            let end: i64 = match arguments.args[1].parse() {
                Ok(value) => value,
                Err(_) => {
                    return CommandResult::Error(
                        format!("Non numeric value: {} provided.", &arguments.args[1]).to_string(),
                    );
                }
            };

            if start > end {
                CommandResult::Error("Invalid arguments provided, range start value cannot be bigger than the range end value.".to_string())
            } else {
                let array: Vec<_> = (start..end)
                    .map(|value| StateValue::Number64Bit(value))
                    .collect();

                let key = put_handle(arguments.state, StateValue::List(array));

                CommandResult::Continue(Some(key))
            }
        }
    }
}

pub(crate) fn create(package: &str) -> Box<dyn Command> {
    Box::new(CommandImpl {
        package: package.to_string(),
    })
}
