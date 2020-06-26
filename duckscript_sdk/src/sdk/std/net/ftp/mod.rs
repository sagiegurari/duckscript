mod list;
mod nlst;

use crate::utils::pckg;
use duckscript::types::command::{CommandResult, Commands};
use duckscript::types::error::ScriptError;
use ftp::FtpStream;

static PACKAGE: &str = "ftp";

pub(crate) fn load(commands: &mut Commands, parent: &str) -> Result<(), ScriptError> {
    let package = pckg::concat(parent, PACKAGE);

    commands.set(list::create(&package))?;
    commands.set(nlst::create(&package))?;

    Ok(())
}

#[derive(Clone)]
pub(crate) struct Options {
    host: Option<String>,
    port: u32,
    user_name: Option<String>,
    password: Option<String>,
    path: Option<String>,
}

impl Options {
    fn new() -> Options {
        Options {
            host: None,
            port: 21,
            user_name: None,
            password: None,
            path: None,
        }
    }
}

enum LookingFor {
    Flag,
    Host,
    Port,
    UserName,
    Password,
    Path,
}

pub(crate) fn run_with_connection(
    arguments: &Vec<String>,
    func: &mut FnMut(&Options, &mut FtpStream) -> CommandResult,
) -> CommandResult {
    match parse_common_options(&arguments) {
        Ok(options) => run_in_ftp_connection_context(
            &options,
            &mut |ftp_stream: &mut FtpStream| -> CommandResult { func(&options, ftp_stream) },
        ),
        Err(error) => CommandResult::Error(error),
    }
}

fn parse_common_options(arguments: &Vec<String>) -> Result<Options, String> {
    let mut options = Options::new();

    let mut looking_for = LookingFor::Flag;
    for argument in arguments {
        match looking_for {
            LookingFor::Flag => match argument.as_str() {
                "--host" => looking_for = LookingFor::Host,
                "--port" => looking_for = LookingFor::Port,
                "--username" => looking_for = LookingFor::UserName,
                "--password" => looking_for = LookingFor::Password,
                "--path" => looking_for = LookingFor::Path,
                _ => (),
            },
            LookingFor::Host => {
                if !argument.is_empty() {
                    options.host = Some(argument.to_string());
                }
                looking_for = LookingFor::Flag;
            }
            LookingFor::Port => {
                if !argument.is_empty() {
                    options.port = match argument.parse::<u32>() {
                        Ok(value) => value,
                        Err(error) => return Err(error.to_string()),
                    };
                }
                looking_for = LookingFor::Flag;
            }
            LookingFor::UserName => {
                if !argument.is_empty() {
                    options.user_name = Some(argument.to_string());
                }
                looking_for = LookingFor::Flag;
            }
            LookingFor::Password => {
                if !argument.is_empty() {
                    options.password = Some(argument.to_string());
                }
                looking_for = LookingFor::Flag;
            }
            LookingFor::Path => {
                if !argument.is_empty() {
                    options.path = Some(argument.to_string());
                }
                looking_for = LookingFor::Flag;
            }
        }
    }

    Ok(options)
}

fn run_in_ftp_connection_context(
    options: &Options,
    func: &mut FnMut(&mut FtpStream) -> CommandResult,
) -> CommandResult {
    match options.host {
        Some(ref host) => {
            let mut connection_string = String::new();
            connection_string.push_str(host);
            connection_string.push(':');
            connection_string.push_str(&options.port.to_string());

            match FtpStream::connect(&connection_string) {
                Ok(mut ftp_stream) => {
                    let options_cloned = options.clone();

                    // login if needed
                    if options.user_name.is_some() && options.password.is_some() {
                        let user_name = options_cloned.user_name.unwrap();
                        let password = options_cloned.password.unwrap();

                        if let Err(error) = ftp_stream.login(&user_name, &password) {
                            return CommandResult::Error(error.to_string());
                        }
                    }

                    // move to another directory
                    if let Some(path) = options_cloned.path {
                        if let Err(error) = ftp_stream.cwd(path.as_str()) {
                            return CommandResult::Error(error.to_string());
                        }
                    }

                    let result = func(&mut ftp_stream);

                    ftp_stream.quit().unwrap_or(());

                    result
                }
                Err(error) => CommandResult::Error(error.to_string()),
            }
        }
        None => CommandResult::Error("No host provided.".to_string()),
    }
}
