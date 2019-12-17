use crate::instruction::InstructionMetaInfo;
use std::fmt;
use std::fmt::Display;
use std::io;

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
    PreProcessNoCommandFound(InstructionMetaInfo),
    ControlWithoutValidValue(InstructionMetaInfo),
    InvalidControlLocation(InstructionMetaInfo),
    MissingEndQuotes(InstructionMetaInfo),
    MissingOutputVariableName(InstructionMetaInfo),
    InvalidEqualsLocation(InstructionMetaInfo),
    InvalidQuotesLocation(InstructionMetaInfo),
    EmptyLabel(InstructionMetaInfo),
    UnknownPreProcessorCommand(InstructionMetaInfo),
    ErrorReadingFile(io::Error),
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
            ErrorInfo::ErrorReadingFile(ref cause) => cause.fmt(formatter),
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
