use duckscript::types::error::{ErrorInfo, ScriptError};
use std::fs::{create_dir_all, remove_file, File, OpenOptions};
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

pub(crate) fn create_parent_directory(file: &str) -> Result<(), String> {
    match get_parent_directory_name(file) {
        Some(parent) => create_directory(&parent),
        None => Ok(()),
    }
}

fn create_directory_for_path(directory_path: &Path) -> Result<(), String> {
    if directory_path.is_dir() && directory_path.exists() {
        return Ok(());
    }

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
            match fd.read_to_string(&mut content) {
                Ok(_) => Ok(content),
                Err(error) => Err(ScriptError {
                    info: ErrorInfo::ErrorReadingFile(file.to_string(), Some(error)),
                }),
            }
        }
        Err(error) => Err(ScriptError {
            info: ErrorInfo::ErrorReadingFile(file.to_string(), Some(error)),
        }),
    }
}

pub(crate) fn read_raw_file(file: &str) -> Result<Vec<u8>, ScriptError> {
    let file_path = Path::new(file);

    match File::open(&file_path) {
        Ok(mut fd) => {
            let mut content = vec![];
            match fd.read_to_end(&mut content) {
                Ok(_) => Ok(content),
                Err(error) => Err(ScriptError {
                    info: ErrorInfo::ErrorReadingFile(file.to_string(), Some(error)),
                }),
            }
        }
        Err(error) => Err(ScriptError {
            info: ErrorInfo::ErrorReadingFile(file.to_string(), Some(error)),
        }),
    }
}

pub(crate) fn write_text_file(file: &str, text: &str) -> Result<(), ScriptError> {
    write_to_text_file(file, text, false)
}

pub(crate) fn write_to_text_file(file: &str, text: &str, append: bool) -> Result<(), ScriptError> {
    write_to_file(file, text.as_bytes(), append)
}

pub(crate) fn write_to_file(file: &str, data: &[u8], append: bool) -> Result<(), ScriptError> {
    let file_path = Path::new(file);

    // create parent directory
    match create_parent_directory(file) {
        Ok(_) => (),
        Err(_) => {
            return Err(ScriptError {
                info: ErrorInfo::Runtime(
                    format!("Error creating parent directory for file: {}", file).to_string(),
                    None,
                ),
            });
        }
    }

    let result = if append && file_path.exists() {
        OpenOptions::new().append(true).open(file)
    } else {
        File::create(&file_path)
    };

    match result {
        Ok(mut fd) => match fd.write_all(data) {
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

pub(crate) fn create_empty_file(file: &str) -> Result<(), String> {
    let file_path = Path::new(file);

    if file_path.exists() {
        if file_path.is_file() {
            Ok(())
        } else {
            Err(format!(
                "Unable to create file: {} directory with that path exists.",
                file
            )
            .to_string())
        }
    } else {
        // create parent directory
        let directory = file_path.parent();
        match directory {
            Some(directory_path) => match create_directory_for_path(&directory_path) {
                _ => (),
            },
            None => (),
        };

        match File::create(&file_path) {
            Ok(_) => Ok(()),
            _ => Err(format!("Unable to create file: {}", file).to_string()),
        }
    }
}

pub(crate) fn delete_file(file: &str) -> Result<(), String> {
    let file_path = Path::new(file);

    if file_path.exists() {
        match remove_file(file) {
            Ok(_) => Ok(()),
            Err(error) => Err(format!(
                "Unable to delete file: {} error: {}",
                file,
                error.to_string()
            )
            .to_string()),
        }
    } else {
        Ok(())
    }
}
