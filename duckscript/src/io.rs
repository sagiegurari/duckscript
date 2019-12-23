//! # io
//!
//! IO helper functions
//!

#[cfg(test)]
#[path = "./io_test.rs"]
mod io_test;

use crate::types::error::{ErrorInfo, ScriptError};
use std::fs::File;
use std::io::Read;
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
