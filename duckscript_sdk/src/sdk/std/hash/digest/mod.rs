use crate::utils::pckg;
use core::fmt::Write;
use duckscript::types::command::{Command, CommandInvocationContext, CommandResult};
use sha2::{Digest, Sha256, Sha512};
use std::fs::File;
use std::io::Read;

#[cfg(test)]
#[path = "./mod_test.rs"]
mod mod_test;

enum Algorithm {
    SHA256,
    SHA512,
}

#[derive(Default)]
struct Options {
    algorithm: Option<Algorithm>,
    file: Option<String>,
    string: Option<String>,
}

impl Options {
    fn new() -> Options {
        Default::default()
    }
}

enum LookingFor {
    Flag,
    Algorithm,
    File,
}

const BUFFER_SIZE: usize = 1024;

fn parse_options(arguments: &Vec<String>) -> Result<Options, String> {
    let mut options = Options::new();

    let mut looking_for = LookingFor::Flag;
    for argument in arguments {
        match looking_for {
            LookingFor::Flag => match argument.as_str() {
                "--algo" => looking_for = LookingFor::Algorithm,
                "--file" => looking_for = LookingFor::File,
                _ => options.string = Some(argument.to_string()),
            },
            LookingFor::Algorithm => {
                if !argument.is_empty() {
                    match argument.to_lowercase().as_str() {
                        "sha256" => options.algorithm = Some(Algorithm::SHA256),
                        "sha512" => options.algorithm = Some(Algorithm::SHA512),
                        _ => {
                            return Err(format!("Unsupported algorithm: {}", argument).to_string());
                        }
                    };
                }

                looking_for = LookingFor::Flag;
            }
            LookingFor::File => {
                if !argument.is_empty() {
                    options.file = Some(argument.to_string());
                }
                looking_for = LookingFor::Flag;
            }
        }
    }

    Ok(options)
}

fn bytes_to_hex(bytes: &[u8]) -> Result<String, String> {
    let mut hex_string = String::with_capacity(2 * bytes.len());
    for byte in bytes {
        match write!(hex_string, "{:02X}", byte) {
            Err(error) => return Err(error.to_string()),
            _ => (),
        }
    }

    Ok(hex_string)
}

fn hash_file<D: Digest + Default>(filename: &str) -> Result<String, String> {
    match File::open(filename) {
        Ok(mut file) => {
            let mut digest = D::new();

            let mut buffer = [0u8; BUFFER_SIZE];
            loop {
                let size = match file.read(&mut buffer) {
                    Ok(size) => size,
                    Err(error) => return Err(error.to_string()),
                };

                digest.update(&buffer[..size]);

                if size == 0 || size < BUFFER_SIZE {
                    break;
                }
            }

            let bytes = digest.finalize();
            bytes_to_hex(&bytes[..])
        }
        Err(error) => Err(error.to_string()),
    }
}

fn hash_string(algorithm: Algorithm, string: &str) -> Result<String, String> {
    let string_bytes = string.as_bytes();
    match algorithm {
        Algorithm::SHA256 => {
            let bytes = Sha256::digest(string_bytes);
            bytes_to_hex(&bytes[..])
        }
        Algorithm::SHA512 => {
            let bytes = Sha512::digest(string_bytes);
            bytes_to_hex(&bytes[..])
        }
    }
}

#[derive(Clone)]
pub(crate) struct CommandImpl {
    package: String,
}

impl Command for CommandImpl {
    fn name(&self) -> String {
        pckg::concat(&self.package, "Digest")
    }

    fn aliases(&self) -> Vec<String> {
        vec!["digest".to_string()]
    }

    fn help(&self) -> String {
        include_str!("help.md").to_string()
    }

    fn clone_and_box(&self) -> Box<dyn Command> {
        Box::new((*self).clone())
    }

    fn run(&self, context: CommandInvocationContext) -> CommandResult {
        if context.arguments.is_empty() {
            CommandResult::Error("No input provided.".to_string())
        } else {
            match parse_options(&context.arguments) {
                Ok(options) => {
                    if options.algorithm.is_none() {
                        CommandResult::Error("No algorithm defined".to_string())
                    } else if options.file.is_none() && options.string.is_none() {
                        CommandResult::Error("No input to hash provided".to_string())
                    } else {
                        let algorithm = options.algorithm.unwrap();

                        if options.file.is_some() {
                            let file = &options.file.unwrap();

                            let result = match algorithm {
                                Algorithm::SHA256 => hash_file::<Sha256>(file),
                                Algorithm::SHA512 => hash_file::<Sha512>(file),
                            };
                            match result {
                                Ok(value) => CommandResult::Continue(Some(value)),
                                Err(error) => CommandResult::Error(error),
                            }
                        } else {
                            match hash_string(algorithm, &options.string.unwrap()) {
                                Ok(value) => CommandResult::Continue(Some(value)),
                                Err(error) => CommandResult::Error(error),
                            }
                        }
                    }
                }
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
