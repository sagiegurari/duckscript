use duckscript::types::error::{ErrorInfo, ScriptError};
use std::fs::{create_dir_all, File};
use std::io::{Read, Write};
use std::path::Path;

#[cfg(test)]
#[path = "./io_test.rs"]
mod io_test;

pub(crate) fn get_canonical_path(path: &str) -> String {
    let path_obj = Path::new(path);

    match path_obj.canonicalize() {
        Ok(path_buf) => path_buf.to_string_lossy().into_owned(),
        _ => path.to_string(),
    }
}

pub(crate) fn get_base_name(path: &str) -> Option<String> {
    let canonical_path = get_canonical_path(path);
    let path = Path::new(&canonical_path);

    match path.file_name() {
        Some(name) => Some(name.to_string_lossy().into_owned()),
        None => None,
    }
}

pub(crate) fn get_parent_directory_name(path: &str) -> Option<String> {
    let file_path = Path::new(path);

    let directory = file_path.parent();
    match directory {
        Some(directory_path) => {
            let directory = directory_path.to_string_lossy().into_owned();

            if directory.is_empty() {
                None
            } else {
                Some(directory)
            }
        }
        None => None,
    }
}

pub(crate) fn create_directory(directory: &str) -> Result<(), String> {
    let directory_path = Path::new(directory);

    create_directory_for_path(&directory_path)
}

fn create_directory_for_path(directory_path: &Path) -> Result<(), String> {
    match create_dir_all(&directory_path) {
        Ok(_) => Ok(()),
        Err(error) => Err(error.to_string()),
    }
}

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

pub(crate) fn write_text_file(file: &str, text: &str) -> Result<(), ScriptError> {
    let file_path = Path::new(file);

    // create parent directory
    let directory = file_path.parent();
    match directory {
        Some(directory_path) => match create_directory_for_path(&directory_path) {
            _ => (),
        },
        None => (),
    };

    match File::create(&file_path) {
        Ok(mut fd) => match fd.write_all(text.as_bytes()) {
            Err(_) => Err(ScriptError {
                info: ErrorInfo::Runtime(
                    format!("Error writing to file: {}", file).to_string(),
                    None,
                ),
            }),
            Ok(_) => match fd.sync_all() {
                _ => Ok(()),
            },
        },
        Err(_) => Err(ScriptError {
            info: ErrorInfo::Runtime(
                format!("Error opening file: {} for writing.", file).to_string(),
                None,
            ),
        }),
    }
}
