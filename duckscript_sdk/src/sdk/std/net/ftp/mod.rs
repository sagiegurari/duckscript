mod get;
mod get_in_memory;
mod list;
mod nlst;
mod put;
mod put_in_memory;

use crate::utils::pckg;
use duckscript::types::command::{CommandResult, Commands};
use duckscript::types::error::ScriptError;
use ftp::types::{FileType, FormatControl};
use ftp::FtpStream;

static PACKAGE: &str = "ftp";

pub(crate) fn load(commands: &mut Commands, parent: &str) -> Result<(), ScriptError> {
    let package = pckg::concat(parent, PACKAGE);

    commands.set(get::create(&package))?;
    commands.set(get_in_memory::create(&package))?;
    commands.set(list::create(&package))?;
    commands.set(nlst::create(&package))?;
    commands.set(put::create(&package))?;
    commands.set(put_in_memory::create(&package))?;

    Ok(())
}

#[derive(Clone)]
pub(crate) enum TransferType {
    // ascii
    Ascii,
    // image/binary
    Binary,
}

#[derive(Clone)]
pub(crate) struct Options {
    host: Option<String>,
    port: u32,
    user_name: Option<String>,
    password: Option<String>,
    path: Option<String>,
    transfer_type: Option<TransferType>,
    remote_file: Option<String>,
    local_file: Option<String>,
    content: Option<String>,
}

impl Options {
    fn new() -> Options {
        Options {
            host: None,
            port: 21,
            user_name: None,
            password: None,
            path: None,
            transfer_type: None,
            remote_file: None,
            local_file: None,
            content: None,
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
    TransferType,
    RemoteFile,
    LocalFile,
    Content,
}

pub(crate) fn run_with_connection(
    arguments: &Vec<String>,
    func: &mut dyn FnMut(&Options, &mut FtpStream) -> CommandResult,
) -> CommandResult {
    validate_and_run_with_connection(
        arguments,
        &|_options: &Options| -> Result<(), String> { Ok(()) },
        func,
    )
}

pub(crate) fn validate_and_run_with_connection(
    arguments: &Vec<String>,
    validate_input: &dyn Fn(&Options) -> Result<(), String>,
    func: &mut dyn FnMut(&Options, &mut FtpStream) -> CommandResult,
) -> CommandResult {
    match parse_common_options(&arguments) {
        Ok(options) => match validate_input(&options) {
            Ok(_) => run_in_ftp_connection_context(
                &options,
                &mut |ftp_stream: &mut FtpStream| -> CommandResult { func(&options, ftp_stream) },
            ),
            Err(error) => CommandResult::Error(error),
        },
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
                "--type" => looking_for = LookingFor::TransferType,
                "--remote-file" => looking_for = LookingFor::RemoteFile,
                "--local-file" => looking_for = LookingFor::LocalFile,
                "--content" => looking_for = LookingFor::Content,
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
            LookingFor::TransferType => {
                if !argument.is_empty() {
                    match argument.as_str() {
                        "A" => options.transfer_type = Some(TransferType::Ascii),
                        "I" => options.transfer_type = Some(TransferType::Binary),
                        _ => {
                            return Err(
                                "Invalid transfer type provided, only A or I are supported."
                                    .to_string(),
                            )
                        }
                    }
                    looking_for = LookingFor::Flag;
                }
            }
            LookingFor::RemoteFile => {
                if !argument.is_empty() {
                    options.remote_file = Some(argument.to_string());
                }
                looking_for = LookingFor::Flag;
            }
            LookingFor::LocalFile => {
                if !argument.is_empty() {
                    options.local_file = Some(argument.to_string());
                }
                looking_for = LookingFor::Flag;
            }
            LookingFor::Content => {
                if !argument.is_empty() {
                    options.content = Some(argument.to_string());
                }
                looking_for = LookingFor::Flag;
            }
        }
    }

    Ok(options)
}

fn run_in_ftp_connection_context(
    options: &Options,
    func: &mut dyn FnMut(&mut FtpStream) -> CommandResult,
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

                    // set transfer type
                    if let Some(transfer_type) = options_cloned.transfer_type {
                        let file_type = match transfer_type {
                            TransferType::Ascii => FileType::Ascii(FormatControl::Default),
                            TransferType::Binary => FileType::Image,
                        };

                        if let Err(error) = ftp_stream.transfer_type(file_type) {
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
