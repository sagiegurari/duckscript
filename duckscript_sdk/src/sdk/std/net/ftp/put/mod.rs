use crate::sdk::std::net::ftp::{validate_and_run_with_connection, Options};
use crate::utils::pckg;
use duckscript::types::command::{Command, CommandInvocationContext, CommandResult};
use fsio::path::as_path::AsPath;
use std::fs::File;
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
        pckg::concat(&self.package, "Put")
    }

    fn aliases(&self) -> Vec<String> {
        vec!["ftp_put".to_string()]
    }

    fn help(&self) -> String {
        include_str!("help.md").to_string()
    }

    fn clone_and_box(&self) -> Box<dyn Command> {
        Box::new((*self).clone())
    }

    fn run(&self, context: CommandInvocationContext) -> CommandResult {
        validate_and_run_with_connection(
            &context.arguments,
            &|options: &Options| -> Result<(), String> {
                if options.remote_file.is_none() {
                    Err("Missing remote file name".to_string())
                } else {
                    let options_clone = options.clone();
                    match options_clone.local_file {
                        Some(local_file) => {
                            let file_path = local_file.as_path();

                            if file_path.exists() {
                                if file_path.is_file() {
                                    Ok(())
                                } else {
                                    Err("Local path is a directory.".to_string())
                                }
                            } else {
                                Err("Local file not found.".to_string())
                            }
                        }
                        None => Err("Missing local file name.".to_string()),
                    }
                }
            },
            &mut |options: &Options, ftp_stream: &mut FtpStream| -> CommandResult {
                let options_clone = options.clone();
                let remote_file = options_clone.remote_file.unwrap();
                let local_file = options_clone.local_file.unwrap();

                match File::open(local_file) {
                    Ok(mut file) => match ftp_stream.put_file(&remote_file, &mut file) {
                        Ok(_) => CommandResult::Continue(Some(true.to_string())),
                        Err(error) => CommandResult::Error(error.to_string()),
                    },
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
