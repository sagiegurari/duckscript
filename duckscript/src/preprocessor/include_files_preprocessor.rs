//! # include_files_preprocessor
//!
//! Enables to load and include additional source files.
//!

use crate::parser;
use crate::types::error::ScriptError;
use crate::types::instruction::{Instruction, InstructionMetaInfo};
use std::path::PathBuf;

pub(crate) fn run(
    arguments: &Option<Vec<String>>,
    meta_info: &InstructionMetaInfo,
) -> Result<Vec<Instruction>, ScriptError> {
    let mut instructions = vec![];

    if let Some(arguments_vec) = arguments {
        for argument in arguments_vec {
            // get file based on parent file location
            let file_path = if argument.starts_with("/") || argument.starts_with("\\") {
                argument.to_string()
            } else {
                match &meta_info.source {
                    Some(value) => {
                        let path_buffer = PathBuf::from(&value);
                        match path_buffer.parent() {
                            Some(path) => {
                                let mut parent_path_buffer = path.to_path_buf();
                                parent_path_buffer.push(&argument);

                                let full_path_buffer = match parent_path_buffer.canonicalize() {
                                    Ok(new_buffer) => new_buffer,
                                    _ => parent_path_buffer,
                                };

                                full_path_buffer.to_string_lossy().into_owned()
                            }
                            _ => argument.to_string(),
                        }
                    }
                    None => argument.to_string(),
                }
            };

            match parser::parse_file(&file_path) {
                Ok(mut additional_instructions) => {
                    instructions.append(&mut additional_instructions)
                }
                Err(error) => return Err(error),
            }
        }
    }

    Ok(instructions)
}
