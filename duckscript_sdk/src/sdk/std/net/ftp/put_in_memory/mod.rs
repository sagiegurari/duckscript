use crate::sdk::std::net::ftp::{validate_and_run_with_connection, Options};
use crate::utils::pckg;
use duckscript::types::command::{Command, CommandArgs, CommandResult};
use std::io::Cursor;
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
        pckg::concat(&self.package, "PutInMemory")
    }

    fn aliases(&self) -> Vec<String> {
        vec!["ftp_put_in_memory".to_string()]
    }

    fn help(&self) -> String {
        include_str!("help.md").to_string()
    }

    fn clone_and_box(&self) -> Box<dyn Command> {
        Box::new((*self).clone())
    }

    fn run(&self, arguments: CommandArgs) -> CommandResult {
        validate_and_run_with_connection(
            &arguments,
            &|options: &Options| -> Result<(), String> {
                if options.remote_file.is_none() {
                    Err("Missing remote file name".to_string())
                } else if options.content.is_none() {
                    Err("Missing content".to_string())
                } else {
                    Ok(())
                }
            },
            &mut |options: &Options, ftp_stream: &mut FtpStream| -> CommandResult {
                let options_clone = options.clone();
                let remote_file = options_clone.remote_file.unwrap();
                let content = options_clone.content.unwrap();

                let mut reader = Cursor::new(content.as_bytes());
                match ftp_stream.put_file(&remote_file, &mut reader) {
                    Ok(_) => CommandResult::Continue(Some(true.to_string())),
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
