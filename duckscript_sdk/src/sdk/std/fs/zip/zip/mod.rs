use crate::utils::{pckg, state};
use duckscript::types::command::{Command, CommandResult, Commands};
use duckscript::types::env::Env;
use duckscript::types::instruction::Instruction;
use duckscript::types::runtime::StateValue;
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io;
use std::path::Path;
use zip::write::SimpleFileOptions;
use zip::{CompressionMethod, ZipWriter};

#[derive(Clone)]
pub(crate) struct CommandImpl {
    package: String,
}

enum LookingFor {
    Options,
    Base,
    Compression,
    Files,
}

impl Command for CommandImpl {
    fn name(&self) -> String {
        pckg::concat(&self.package, "Zip")
    }

    fn aliases(&self) -> Vec<String> {
        vec!["zip".to_string()]
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
        _env: &mut Env,
    ) -> CommandResult {
        if arguments.len() < 2 {
            return CommandResult::Error(
                "Paths to the ZIP file and/or files to pack are not provided.".to_string(),
            );
        }

        let CommandOptions {
            append,
            base,
            compression,
            zipfile,
            file_args,
        } = match parse_args(&arguments) {
            Ok(options) => options,
            Err(err) => return CommandResult::Error(err),
        };

        let files = match collect_files_from_arrays(&file_args, state) {
            Ok(files) => files,
            Err(err) => return CommandResult::Error(err),
        };

        let zipfile = Path::new(zipfile);
        let zipfile_dir = match zipfile.parent() {
            Some(path) => path,
            None => return CommandResult::Error("Couldn't parse ZIP file directory.".to_string()),
        };
        match std::fs::create_dir_all(zipfile_dir) {
            Ok(_) => (),
            Err(err) => {
                return CommandResult::Error(format!("Couldn't create ZIP file directory: {}", err))
            }
        };
        let zip_file_existed = zipfile.exists();
        let zip_file = match OpenOptions::new()
            .read(true)
            .write(true)
            .create(!zip_file_existed)
            .truncate(!append)
            .open(zipfile)
        {
            Ok(file) => file,
            Err(err) => {
                return CommandResult::Error(format!("Couldn't create/open ZIP file: {}", err))
            }
        };
        let mut zip = if append && zip_file_existed {
            match ZipWriter::new_append(zip_file) {
                Ok(writer) => writer,
                Err(err) => {
                    return CommandResult::Error(format!("Couldn't open ZIP file: {}", err))
                }
            }
        } else {
            ZipWriter::new(zip_file)
        };

        let zip_options = SimpleFileOptions::default()
            .compression_method(compression)
            .unix_permissions(0o755);

        for file_to_add_str in files {
            let file_to_add_path = Path::new(&file_to_add_str);
            let mut file_to_add = match File::open(file_to_add_path) {
                Ok(file) => file,
                Err(err) => {
                    return CommandResult::Error(format!(
                        "File does not exist or can't be opened: {} ({})",
                        file_to_add_str, err
                    ))
                }
            };

            let file_to_add_path_stripped = file_to_add_path
                .strip_prefix("./")
                .unwrap_or(file_to_add_path);

            let file_to_add_path_stripped = match base {
                Some(base) => file_to_add_path_stripped
                    .strip_prefix(base)
                    .unwrap_or(file_to_add_path_stripped),
                None => file_to_add_path_stripped,
            };

            let file_to_add_path_str = match file_to_add_path_stripped.to_str() {
                Some(str) => str,
                None => return CommandResult::Error("Invalid file path".to_string()),
            };

            match zip.start_file(file_to_add_path_str, zip_options) {
                Ok(_) => (),
                Err(err) => {
                    return CommandResult::Error(format!(
                        "Could not write file to archive: {} ({})",
                        file_to_add_str, err
                    ))
                }
            };

            match io::copy(&mut file_to_add, &mut zip) {
                Ok(_) => (),
                Err(err) => {
                    return CommandResult::Error(format!(
                        "Could not write file to archive: {} ({})",
                        file_to_add_str, err
                    ))
                }
            }
        }

        match zip.finish() {
            Ok(_) => (),
            Err(err) => {
                return CommandResult::Error(format!("Could not finish the archive: {}", err))
            }
        };

        CommandResult::Continue(Some("true".to_string()))
    }
}

struct CommandOptions<'a> {
    append: bool,
    base: Option<&'a str>,
    compression: CompressionMethod,
    zipfile: &'a str,
    file_args: Vec<&'a str>,
}

fn parse_args<'a>(arguments: &'a [String]) -> Result<CommandOptions<'a>, String> {
    let mut looking_for = LookingFor::Options;
    let mut append = false;
    let mut base = None;
    let mut compression = CompressionMethod::Deflated;
    let mut zipfile = None;
    let mut file_args = Vec::new();

    for argument in arguments {
        match looking_for {
            LookingFor::Options => match argument.as_str() {
                "--compression" => looking_for = LookingFor::Compression,
                "--base" => looking_for = LookingFor::Base,
                "--append" => append = true,
                _ => {
                    zipfile = Some(argument.as_str());
                    looking_for = LookingFor::Files;
                }
            },

            LookingFor::Compression => {
                match argument.as_str() {
                    "deflate" => compression = CompressionMethod::Deflated,
                    "bzip2" => compression = CompressionMethod::Bzip2,
                    "none" => compression = CompressionMethod::Stored,
                    _ => return Err("Unknown compression method.".to_string()),
                }

                looking_for = LookingFor::Options;
            }

            LookingFor::Base => {
                let base_str = argument.as_str();
                base = Some(base_str.strip_prefix("./").unwrap_or(base_str));
                looking_for = LookingFor::Options;
            }

            LookingFor::Files => file_args.push(argument.as_str()),
        }
    }

    if file_args.is_empty() {
        return Err("Input files not provided.".to_string());
    }

    let zipfile = match zipfile {
        Some(filename) => filename,
        None => return Err("ZIP file name not provided.".to_string()),
    };

    Ok(CommandOptions {
        append,
        base,
        compression,
        zipfile,
        file_args,
    })
}

fn collect_files_from_arrays(
    files: &[&str],
    state: &mut HashMap<String, StateValue>,
) -> Result<Vec<String>, String> {
    let mut collected = Vec::new();

    for arg in files {
        match state::get_handle(state, arg.to_string()) {
            Some(value) => match value {
                StateValue::List(entries) => {
                    for entry in entries {
                        collected.push(state::get_as_string(entry)?);
                    }
                }

                arg => collected.push(state::get_as_string(arg)?),
            },

            None => collected.push(arg.to_string()),
        };
    }

    Ok(collected)
}

pub(crate) fn create(package: &str) -> Box<dyn Command> {
    Box::new(CommandImpl {
        package: package.to_string(),
    })
}
