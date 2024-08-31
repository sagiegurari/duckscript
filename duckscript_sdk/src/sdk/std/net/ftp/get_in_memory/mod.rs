use crate::sdk::std::net::ftp::{validate_and_run_with_connection, Options};
use crate::utils::pckg;
use crate::utils::state::put_handle;
use duckscript::types::command::{Command, CommandResult, Commands};
use duckscript::types::env::Env;
use duckscript::types::instruction::Instruction;
use duckscript::types::runtime::StateValue;
use std::collections::HashMap;
use suppaftp::FtpStream;

#[cfg(test)]
#[path = "./mod_test.rs"]
mod mod_test;

#[derive(Clone)]
pub(crate) struct CommandImpl {
    package: String,
}

impl Command for CommandImpl {
    fn name(&self) -> String {
        pckg::concat(&self.package, "GetInMemory")
    }

    fn aliases(&self) -> Vec<String> {
        vec!["ftp_get_in_memory".to_string()]
    }

    fn help(&self) -> String {
        include_str!("help.md").to_string()
    }

    fn clone_and_box(&self) -> Box<dyn Command> {
        Box::new((*self).clone())
    }

    fn requires_context(&self) -> bool {
        true
    }

    fn run_with_context(
        &self,
        arguments: Vec<String>,
        state: &mut HashMap<String, StateValue>,
        _variables: &mut HashMap<String, String>,
        _output_variable: Option<String>,
        _instructions: &Vec<Instruction>,
        _commands: &mut Commands,
        _line: usize,
        _env: &mut Env,
    ) -> CommandResult {
        validate_and_run_with_connection(
            &arguments,
            &|options: &Options| -> Result<(), String> {
                if options.remote_file.is_none() {
                    Err("Missing remote file name".to_string())
                } else {
                    Ok(())
                }
            },
            &mut |options: &Options, ftp_stream: &mut FtpStream| -> CommandResult {
                let options_clone = options.clone();
                let remote_file = options_clone.remote_file.unwrap();

                match ftp_stream.retr_as_buffer(&remote_file) {
                    Ok(binary) => {
                        let key = put_handle(state, StateValue::ByteArray(binary.into_inner()));

                        CommandResult::Continue(Some(key))
                    }
                    Err(error) => CommandResult::Error(error.to_string()),
                }
            },
        )
    }
}

pub(crate) fn create(package: &str) -> Box<dyn Command> {
    Box::new(CommandImpl {
        package: package.to_string(),
    })
}
