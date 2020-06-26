use crate::sdk::std::net::ftp::{run_with_connection, Options};
use crate::utils::pckg;
use crate::utils::state::put_handle;
use duckscript::types::command::{Command, CommandResult, Commands};
use duckscript::types::instruction::Instruction;
use duckscript::types::runtime::StateValue;
use ftp::FtpStream;
use std::collections::HashMap;

#[cfg(test)]
#[path = "./mod_test.rs"]
mod mod_test;

#[derive(Clone)]
pub(crate) struct CommandImpl {
    package: String,
}

impl Command for CommandImpl {
    fn name(&self) -> String {
        pckg::concat(&self.package, "NLst")
    }

    fn aliases(&self) -> Vec<String> {
        vec!["ftp_nlst".to_string()]
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
    ) -> CommandResult {
        run_with_connection(&arguments, &mut |options: &Options,
                                              ftp_stream: &mut FtpStream|
         -> CommandResult {
            let options_clone = options.clone();

            let operation_result = match options_clone.path {
                Some(value) => ftp_stream.nlst(Some(value.as_str())),
                None => ftp_stream.nlst(None),
            };

            match operation_result {
                Ok(output) => {
                    let mut array = vec![];

                    for item in output {
                        array.push(StateValue::String(item));
                    }

                    let key = put_handle(state, StateValue::List(array));

                    CommandResult::Continue(Some(key))
                }
                Err(error) => CommandResult::Error(error.to_string()),
            }
        })
    }
}

pub(crate) fn create(package: &str) -> Box<dyn Command> {
    Box::new(CommandImpl {
        package: package.to_string(),
    })
}
