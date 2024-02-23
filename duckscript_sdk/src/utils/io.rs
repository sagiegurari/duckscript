use duckscript::types::error::ScriptError;
use fsio::file::{append_file, ensure_exists, read_file, write_file};

#[cfg(test)]
#[path = "./io_test.rs"]
mod io_test;

pub(crate) fn read_text_file(file: &str) -> Result<String, ScriptError> {
    match fsio::file::read_text_file(file) {
        Ok(content) => Ok(content),
        Err(error) => Err(ScriptError::ErrorReadingFile(file.to_string(), Some(error))),
    }
}

pub(crate) fn read_raw_file(file: &str) -> Result<Vec<u8>, ScriptError> {
    match read_file(file) {
        Ok(content) => Ok(content),
        Err(error) => Err(ScriptError::ErrorReadingFile(file.to_string(), Some(error))),
    }
}

pub(crate) fn write_text_file(file: &str, text: &str) -> Result<(), ScriptError> {
    write_to_text_file(file, text, false)
}

pub(crate) fn write_to_text_file(file: &str, text: &str, append: bool) -> Result<(), ScriptError> {
    write_to_file(file, text.as_bytes(), append)
}

pub(crate) fn write_to_file(file: &str, data: &[u8], append: bool) -> Result<(), ScriptError> {
    let result = if append {
        append_file(file, data)
    } else {
        write_file(file, data)
    };

    match result {
        Ok(content) => Ok(content),
        Err(error) => Err(ScriptError::Runtime(error.to_string(), None)),
    }
}

pub(crate) fn create_empty_file(file: &str) -> Result<(), String> {
    match ensure_exists(file) {
        Ok(_) => Ok(()),
        Err(error) => Err(error.to_string()),
    }
}

pub(crate) fn get_last_modified_time(path: &str) -> Result<u128, String> {
    match fsio::path::get_last_modified_time(path) {
        Ok(time) => Ok(time),
        Err(error) => Err(error.to_string()),
    }
}

pub(crate) fn get_file_size(path: &str) -> Result<u64, String> {
    match std::fs::metadata(path) {
        Ok(metadata) => {
            if metadata.is_file() {
                Ok(metadata.len())
            } else {
                Err("The provided path is not a file.".to_string())
            }
        }
        Err(_error) => Err("Unable to extract metadata for path.".to_string()),
    }
}

pub(crate) fn ends_with_separator(path_str: &str) -> bool {
    path_str.ends_with("/") || path_str.ends_with("\\")
}
