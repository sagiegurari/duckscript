use crate::utils::state::put_handle;
use crate::utils::{io, pckg};
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
        pckg::concat(&self.package, "ReadBytes")
    }

    fn aliases(&self) -> Vec<String> {
        vec!["readbinfile".to_string(), "read_binary_file".to_string()]
    }

    fn help(&self) -> String {
        include_str!("help.md").to_string()
    }

    fn clone_and_box(&self) -> Box<dyn Command> {
        Box::new((*self).clone())
    }

    fn run(&self, arguments: CommandArgs) -> CommandResult {
        if arguments.args.is_empty() {
            CommandResult::Error("File name not provided.".to_string())
        } else {
            let result = io::read_raw_file(&arguments.args[0]);

            match result {
                Ok(binary) => {
                    let key = put_handle(arguments.state, StateValue::ByteArray(binary));

                    CommandResult::Continue(Some(key))
                }
                Err(error) => CommandResult::Error(error.to_string()),
            }
        }
    }
}

pub(crate) fn create(package: &str) -> Box<dyn Command> {
    Box::new(CommandImpl {
        package: package.to_string(),
    })
}
