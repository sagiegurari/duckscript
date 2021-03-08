//! # error
//!
//! The error structure and types.
//!

use crate::types::instruction::InstructionMetaInfo;
use fsio::error::FsIOError;
use std::error::Error;
use std::fmt;
use std::fmt::Display;

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
pub enum ScriptError {
    /// Error Info Type
    ErrorReadingFile(String, Option<FsIOError>),
    /// Error Info Type
    Initialization(String),
    /// Error Info Type
    Runtime(String, Option<InstructionMetaInfo>),
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

impl Display for ScriptError {
    /// Formats the script error using the given formatter.
    fn fmt(&self, formatter: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            Self::ErrorReadingFile(ref file, ref cause) => {
                writeln!(formatter, "Error reading file: {}", file)?;
                match cause {
                    Some(cause_err) => cause_err.fmt(formatter),
                    None => Ok(()),
                }
            }
            Self::Initialization(ref message) => write!(formatter, "{}", message),
            Self::Runtime(ref message, ref meta_info) => {
                let empty_meta_data = InstructionMetaInfo::new();
                let meta_info_value = meta_info.as_ref().unwrap_or(&empty_meta_data);
                format_error_message(formatter, &meta_info_value, message)
            }
            Self::PreProcessNoCommandFound(ref meta_info) => {
                format_error_message(formatter, &meta_info, "preprocessor is missing a command")
            }
            Self::ControlWithoutValidValue(ref meta_info) => format_error_message(
                formatter,
                &meta_info,
                "control character found without a valid value",
            ),
            Self::InvalidControlLocation(ref meta_info) => {
                format_error_message(formatter, &meta_info, "invalid control character location")
            }
            Self::MissingEndQuotes(ref meta_info) => {
                format_error_message(formatter, &meta_info, "missing end quotes")
            }
            Self::MissingOutputVariableName(ref meta_info) => {
                format_error_message(formatter, &meta_info, "missing variable name")
            }
            Self::InvalidEqualsLocation(ref meta_info) => {
                format_error_message(formatter, &meta_info, "invalid equals sign location")
            }
            Self::InvalidQuotesLocation(ref meta_info) => {
                format_error_message(formatter, &meta_info, "invalid quotes location")
            }
            Self::EmptyLabel(ref meta_info) => {
                format_error_message(formatter, &meta_info, "empty lable found")
            }
            Self::UnknownPreProcessorCommand(ref meta_info) => {
                format_error_message(formatter, &meta_info, "unknow preprocessor command")
            }
        }
    }
}

impl Error for ScriptError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::ErrorReadingFile(_, error) => error.as_ref().map(|fsio_error| {
                let std_error: &dyn Error = fsio_error;
                std_error
            }),
            Self::Initialization(_) => None,
            Self::Runtime(_, _) => None,
            Self::PreProcessNoCommandFound(_) => None,
            Self::ControlWithoutValidValue(_) => None,
            Self::InvalidControlLocation(_) => None,
            Self::MissingEndQuotes(_) => None,
            Self::MissingOutputVariableName(_) => None,
            Self::InvalidEqualsLocation(_) => None,
            Self::InvalidQuotesLocation(_) => None,
            Self::EmptyLabel(_) => None,
            Self::UnknownPreProcessorCommand(_) => None,
        }
    }
}
