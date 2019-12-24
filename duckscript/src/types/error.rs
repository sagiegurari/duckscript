//! # error
//!
//! The error structure and types.
//!

use crate::types::instruction::InstructionMetaInfo;
use std::fmt;
use std::fmt::Display;
use std::io;

#[cfg(test)]
#[path = "./error_test.rs"]
mod error_test;

fn format_error_message(
    formatter: &mut fmt::Formatter,
    meta_info: &InstructionMetaInfo,
    message: &str,
) -> Result<(), fmt::Error> {
    let source = match meta_info.source {
        Some(ref value) => value.to_string(),
        None => "Unknown".to_string(),
    };
    let line = match meta_info.line {
        Some(value) => value.to_string(),
        None => "Unknown".to_string(),
    };

    write!(formatter, "Source: {} Line: {} - {}", source, line, message)
}

#[derive(Debug)]
/// Holds the error information
pub enum ErrorInfo {
    /// Error Info Type
    ErrorReadingFile(String, Option<io::Error>),
    /// Error Info Type
    Initialization(String),
    /// Error Info Type
    Runtime(String, InstructionMetaInfo),
    /// Error Info Type
    PreProcessNoCommandFound(InstructionMetaInfo),
    /// Error Info Type
    ControlWithoutValidValue(InstructionMetaInfo),
    /// Error Info Type
    InvalidControlLocation(InstructionMetaInfo),
    /// Error Info Type
    MissingEndQuotes(InstructionMetaInfo),
    /// Error Info Type
    MissingOutputVariableName(InstructionMetaInfo),
    /// Error Info Type
    InvalidEqualsLocation(InstructionMetaInfo),
    /// Error Info Type
    InvalidQuotesLocation(InstructionMetaInfo),
    /// Error Info Type
    EmptyLabel(InstructionMetaInfo),
    /// Error Info Type
    UnknownPreProcessorCommand(InstructionMetaInfo),
}

#[derive(Debug)]
/// Script error struct
pub struct ScriptError {
    /// Holds the error information
    pub info: ErrorInfo,
}

impl Display for ScriptError {
    /// Formats the script error using the given formatter.
    fn fmt(&self, formatter: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self.info {
            ErrorInfo::ErrorReadingFile(ref file, ref cause) => {
                writeln!(formatter, "Error reading file: {}", file)?;
                match cause {
                    Some(cause_err) => cause_err.fmt(formatter),
                    None => Ok(()),
                }
            }
            ErrorInfo::Initialization(ref message) => write!(formatter, "{}", message),
            ErrorInfo::Runtime(ref message, ref meta_info) => {
                format_error_message(formatter, &meta_info, message)
            }
            ErrorInfo::PreProcessNoCommandFound(ref meta_info) => {
                format_error_message(formatter, &meta_info, "preprocessor is missing a command")
            }
            ErrorInfo::ControlWithoutValidValue(ref meta_info) => format_error_message(
                formatter,
                &meta_info,
                "control character found without a valid value",
            ),
            ErrorInfo::InvalidControlLocation(ref meta_info) => {
                format_error_message(formatter, &meta_info, "invalid control character location")
            }
            ErrorInfo::MissingEndQuotes(ref meta_info) => {
                format_error_message(formatter, &meta_info, "missing end quotes")
            }
            ErrorInfo::MissingOutputVariableName(ref meta_info) => {
                format_error_message(formatter, &meta_info, "missing variable name")
            }
            ErrorInfo::InvalidEqualsLocation(ref meta_info) => {
                format_error_message(formatter, &meta_info, "invalid equals sign location")
            }
            ErrorInfo::InvalidQuotesLocation(ref meta_info) => {
                format_error_message(formatter, &meta_info, "invalid quotes location")
            }
            ErrorInfo::EmptyLabel(ref meta_info) => {
                format_error_message(formatter, &meta_info, "empty lable found")
            }
            ErrorInfo::UnknownPreProcessorCommand(ref meta_info) => {
                format_error_message(formatter, &meta_info, "unknow preprocessor command")
            }
        }
    }
}
