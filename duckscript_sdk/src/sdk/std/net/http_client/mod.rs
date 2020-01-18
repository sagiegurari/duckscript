use crate::utils::{io, pckg};
use attohttpc;
use duckscript::types::command::{Command, CommandResult};
use std::fs::File;

#[cfg(test)]
#[path = "./mod_test.rs"]
mod mod_test;

static HTTP_METHOD_PREFIX: &str = "--method=HTTP-";
static POST_DATA_PREFIX: &str = "--post-data=";

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
}

fn parse_options(arguments: &Vec<String>) -> Result<Options, String> {
    let mut options = Options::new();

    let mut looking_for = LookingFor::Flag;
    for argument in arguments {
        match looking_for {
            LookingFor::Flag => match argument.as_str() {
                "-O" => looking_for = LookingFor::OutputFile,
                _ => {
                    if argument.starts_with(HTTP_METHOD_PREFIX) {
                        let method = argument[HTTP_METHOD_PREFIX.len()..].to_lowercase();

                        match method.as_str() {
                            "get" => options.method = Method::Get,
                            "post" => options.method = Method::Post,
                            _ => {
                                return Err(
                                    format!("Unsupported HTTP method: {}", method).to_string()
                                );
                            }
                        }
                    } else if argument.starts_with(POST_DATA_PREFIX) {
                        let payload = argument[POST_DATA_PREFIX.len()..].to_string();

                        options.payload = Some(payload);
                    }

                    looking_for = LookingFor::Flag
                }
            },
            LookingFor::OutputFile => options.output_file = Some(argument.to_string()),
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
                    Some(file) => match io::create_parent_directory(&file) {
                        Ok(_) => match io::delete_file(&file) {
                            Ok(_) => match File::create(file) {
                                Ok(file_struct) => match response.write_to(file_struct) {
                                    Ok(size) => CommandResult::Continue(Some(size.to_string())),
                                    Err(error) => CommandResult::Error(error.to_string()),
                                },
                                Err(error) => CommandResult::Error(error.to_string()),
                            },
                            Err(error) => CommandResult::Error(error),
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

struct CommandImpl {
    package: String,
}

impl Command for CommandImpl {
    fn name(&self) -> String {
        pckg::concat(&self.package, "HttpClient")
    }

    fn aliases(&self) -> Vec<String> {
        vec!["http_client".to_string(), "wget".to_string()]
    }

    fn help(&self) -> String {
        include_str!("help.md").to_string()
    }

    fn run(&self, arguments: Vec<String>) -> CommandResult {
        if arguments.is_empty() {
            CommandResult::Error("URL not provided.".to_string())
        } else {
            let len = arguments.len() - 1;
            let url = arguments[len].to_string();

            match parse_options(&arguments[0..len].to_vec()) {
                Ok(options) => do_request(url, options),
                Err(error) => CommandResult::Error(error),
            }
        }
    }
}

pub(crate) fn create(package: &str) -> Box<dyn Command> {
    Box::new(CommandImpl {
        package: package.to_string(),
    })
}
