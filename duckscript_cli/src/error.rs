//! # error
//!
//! The error structure and types.
//!

use duckscript::types::error::ScriptError;
use std::fmt;
use std::fmt::Display;

#[cfg(test)]
#[path = "./error_test.rs"]
mod error_test;

#[derive(Debug)]
/// Holds the error information
pub(crate) enum ErrorInfo {
    /// Error Info Type
    Script(ScriptError),
    /// Error Info Type
    Cli(&'static str),
}

#[derive(Debug)]
/// Cli error struct
pub(crate) struct CliError {
    /// Holds the error information
    pub(crate) info: ErrorInfo,
}

impl Display for CliError {
    /// Formats the script error using the given formatter.
    fn fmt(&self, formatter: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self.info {
            ErrorInfo::Script(ref cause) => cause.fmt(formatter),
            ErrorInfo::Cli(ref message) => write!(formatter, "{}", message),
        }
    }
}
