use crate::sdk::std::net::ftp::{validate_and_run_with_connection, Options};
use crate::utils::io::create_empty_file;
use crate::utils::pckg;
use duckscript::types::command::{Command, CommandArgs, CommandResult};
use std::fs::OpenOptions;
use std::io::{BufWriter, Error, Read, Write};
use suppaftp::{FtpError, FtpStream};

#[cfg(test)]
#[path = "./mod_test.rs"]
mod mod_test;

fn write_file(reader: &mut dyn Read, target_file: &str) -> Result<(), Error> {
    let mut file = OpenOptions::new().append(true).open(target_file)?;
    {
        let mut writer = BufWriter::new(&mut file);

        let mut buffer = [0; 10240];
        loop {
            let read_size = reader.read(&mut buffer)?;
            if read_size > 0 {
                writer.write_all(&buffer[0..read_size])?;
            } else {
                break;
            }
        }

        writer.flush()?;
    }
    file.sync_all()?;

    Ok(())
}

#[derive(Clone)]
pub(crate) struct CommandImpl {
    package: String,
}

impl Command for CommandImpl {
    fn name(&self) -> String {
        pckg::concat(&self.package, "Get")
    }

    fn aliases(&self) -> Vec<String> {
        vec!["ftp_get".to_string()]
    }

    fn help(&self) -> String {
        include_str!("help.md").to_string()
    }

    fn clone_and_box(&self) -> Box<dyn Command> {
        Box::new((*self).clone())
    }

    fn run(&self, arguments: CommandArgs) -> CommandResult {
        validate_and_run_with_connection(
            &arguments.args,
            &|options: &Options| -> Result<(), String> {
                if options.remote_file.is_none() {
                    Err("Missing remote file name".to_string())
                } else if options.local_file.is_none() {
                    Err("Missing local file name.".to_string())
                } else {
                    Ok(())
                }
            },
            &mut |options: &Options, ftp_stream: &mut FtpStream| -> CommandResult {
                let options_clone = options.clone();
                let remote_file = options_clone.remote_file.unwrap();
                let local_file = options_clone.local_file.unwrap();

                match create_empty_file(&local_file) {
                    Ok(_) => {
                        match ftp_stream.retr(&remote_file, |reader| {
                            match write_file(reader, &local_file) {
                                Ok(_) => Ok(()),
                                Err(error) => Err(FtpError::ConnectionError(error)),
                            }
                        }) {
                            Ok(_) => CommandResult::Continue(Some(true.to_string())),
                            Err(error) => CommandResult::Error(error.to_string()),
                        }
                    }
                    Err(error) => CommandResult::Error(error),
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
