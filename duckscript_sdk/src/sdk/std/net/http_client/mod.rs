use crate::utils::pckg;
use duckscript::types::command::{Command, CommandResult};
use fsio::directory::create_parent;
use fsio::file::delete;
use std::fs::File;

#[cfg(test)]
#[path = "./mod_test.rs"]
mod mod_test;

enum Method {
    Get,
    Post,
}

struct Options {
    method: Method,
    payload: Option<String>,
    output_file: Option<String>,
}

impl Options {
    fn new() -> Options {
        Options {
            method: Method::Get,
            payload: None,
            output_file: None,
        }
    }
}

enum LookingFor {
    Flag,
    OutputFile,
    Method,
    Payload,
}

fn parse_options(arguments: &Vec<String>) -> Result<Options, String> {
    let mut options = Options::new();

    let mut looking_for = LookingFor::Flag;
    for argument in arguments {
        match looking_for {
            LookingFor::Flag => match argument.as_str() {
                "--output-file" => looking_for = LookingFor::OutputFile,
                "--method" => looking_for = LookingFor::Method,
                "--payload" => looking_for = LookingFor::Payload,
                _ => (),
            },
            LookingFor::OutputFile => {
                if !argument.is_empty() {
                    options.output_file = Some(argument.to_string());
                }
                looking_for = LookingFor::Flag;
            }
            LookingFor::Method => {
                if !argument.is_empty() {
                    match argument.to_lowercase().as_str() {
                        "get" => options.method = Method::Get,
                        "post" => options.method = Method::Post,
                        _ => {
                            return Err(
                                format!("Unsupported HTTP method: {}", argument).to_string()
                            );
                        }
                    };
                }

                looking_for = LookingFor::Flag;
            }
            LookingFor::Payload => {
                if !argument.is_empty() {
                    options.payload = Some(argument.to_string());
                }
                looking_for = LookingFor::Flag;
            }
        }
    }

    Ok(options)
}

fn do_request(url: String, options: Options) -> CommandResult {
    let request = match options.method {
        Method::Get => attohttpc::get(url),
        Method::Post => attohttpc::post(url),
    };

    let response_result = match options.method {
        Method::Get => request.send(),
        Method::Post => match options.payload {
            Some(ref payload) => request.text(payload).send(),
            None => request.send(),
        },
    };

    match response_result {
        Ok(response) => {
            if response.is_success() {
                match options.output_file {
                    Some(file) => match create_parent(&file) {
                        Ok(_) => match delete(&file) {
                            Ok(_) => match File::create(file) {
                                Ok(file_struct) => match response.write_to(file_struct) {
                                    Ok(size) => CommandResult::Continue(Some(size.to_string())),
                                    Err(error) => CommandResult::Error(error.to_string()),
                                },
                                Err(error) => CommandResult::Error(error.to_string()),
                            },
                            Err(error) => CommandResult::Error(error.to_string()),
                        },
                        Err(error) => CommandResult::Error(error.to_string()),
                    },
                    None => match response.text() {
                        Ok(text) => CommandResult::Continue(Some(text.to_string())),
                        Err(error) => CommandResult::Error(error.to_string()),
                    },
                }
            } else {
                CommandResult::Error(
                    format!("Error response, status code: {:?}", response.status()).to_string(),
                )
            }
        }
        Err(error) => CommandResult::Error(error.to_string()),
    }
}

#[derive(Clone)]
pub(crate) struct CommandImpl {
    package: String,
}

impl Command for CommandImpl {
    fn name(&self) -> String {
        pckg::concat(&self.package, "HttpClient")
    }

    fn aliases(&self) -> Vec<String> {
        vec!["http_client".to_string()]
    }

    fn help(&self) -> String {
        include_str!("help.md").to_string()
    }

    fn clone_and_box(&self) -> Box<dyn Command> {
        Box::new((*self).clone())
    }

    fn run(&self, arguments: Vec<String>) -> CommandResult {
        if arguments.is_empty() {
            CommandResult::Error("URL not provided.".to_string())
        } else {
            let len = arguments.len() - 1;
            let url = arguments[len].to_string();

            let url_lowercase = url.to_lowercase();
            if !url_lowercase.starts_with("http://") && !url_lowercase.starts_with("https://") {
                CommandResult::Error(format!("Invalid URL: {} provided.", &url).to_string())
            } else {
                match parse_options(&arguments[0..len].to_vec()) {
                    Ok(options) => do_request(url, options),
                    Err(error) => CommandResult::Error(error),
                }
            }
        }
    }
}

pub(crate) fn create(package: &str) -> Box<dyn Command> {
    Box::new(CommandImpl {
        package: package.to_string(),
    })
}
