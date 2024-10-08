use crate::sdk::std::net::ftp::{run_with_connection, Options};
use crate::utils::pckg;
use crate::utils::state::put_handle;
use duckscript::types::command::{Command, CommandInvocationContext, CommandResult};
use duckscript::types::runtime::StateValue;
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

    fn run(&self, context: CommandInvocationContext) -> CommandResult {
        run_with_connection(&context.arguments, &mut |_options: &Options,
                                                      ftp_stream: &mut FtpStream|
         -> CommandResult {
            match ftp_stream.nlst(None) {
                Ok(output) => {
                    let mut array = vec![];

                    for item in output {
                        array.push(StateValue::String(item));
                    }

                    let key = put_handle(context.state, StateValue::List(array));

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
