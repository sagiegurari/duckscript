use duckscript::types::error::{ErrorInfo, ScriptError};
use duckscript::types::instruction::InstructionMetaInfo;
use std::fs::{create_dir_all, File};
use std::io::{Read, Write};
use std::path::Path;

pub(crate) fn read_text_file(file: &str) -> Result<String, ScriptError> {
    let file_path = Path::new(file);

    match File::open(&file_path) {
        Ok(mut fd) => {
            let mut content = String::new();
            fd.read_to_string(&mut content).unwrap();

            Ok(content)
        }
        Err(error) => Err(ScriptError {
            info: ErrorInfo::ErrorReadingFile(file.to_string(), Some(error)),
        }),
    }
}

pub(crate) fn write_text_file(
    file: &str,
    text: &str,
    meta_info: &InstructionMetaInfo,
) -> Result<(), ScriptError> {
    let file_path = Path::new(file);

    // create parent directory
    let directory = file_path.parent();
    match directory {
        Some(directory_path) => match create_dir_all(&directory_path) {
            _ => (),
        },
        None => (),
    };

    match File::create(&file_path) {
        Ok(mut fd) => match fd.write_all(text.as_bytes()) {
            Err(_) => Err(ScriptError {
                info: ErrorInfo::Runtime(
                    format!("Error writing to file: {}", file).to_string(),
                    meta_info.clone(),
                ),
            }),
            Ok(_) => match fd.sync_all() {
                _ => Ok(()),
            },
        },
        Err(_) => Err(ScriptError {
            info: ErrorInfo::Runtime(
                format!("Error opening file: {} for writing.", file).to_string(),
                meta_info.clone(),
            ),
        }),
    }
}
