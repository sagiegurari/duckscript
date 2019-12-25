use super::*;

#[test]
fn display_error_reading_file() {
    let error = ScriptError {
        info: ErrorInfo::ErrorReadingFile("test".to_string(), None),
    };
    println!("{}", error);
}

#[test]
fn display_initialization() {
    let error = ScriptError {
        info: ErrorInfo::Initialization("test".to_string()),
    };
    println!("{}", error);
}

#[test]
fn display_runtime_with_meta_info() {
    let error = ScriptError {
        info: ErrorInfo::Runtime("test".to_string(), Some(InstructionMetaInfo::new())),
    };
    println!("{}", error);
}

#[test]
fn display_runtime_without_meta_info() {
    let error = ScriptError {
        info: ErrorInfo::Runtime("test".to_string(), None),
    };
    println!("{}", error);
}

#[test]
fn display_pre_process_no_command_found() {
    let error = ScriptError {
        info: ErrorInfo::PreProcessNoCommandFound(InstructionMetaInfo::new()),
    };
    println!("{}", error);
}

#[test]
fn display_control_without_valid_value() {
    let error = ScriptError {
        info: ErrorInfo::ControlWithoutValidValue(InstructionMetaInfo::new()),
    };
    println!("{}", error);
}

#[test]
fn display_invalid_control_location() {
    let error = ScriptError {
        info: ErrorInfo::InvalidControlLocation(InstructionMetaInfo::new()),
    };
    println!("{}", error);
}

#[test]
fn display_missing_end_quotes() {
    let error = ScriptError {
        info: ErrorInfo::MissingEndQuotes(InstructionMetaInfo::new()),
    };
    println!("{}", error);
}

#[test]
fn display_missing_output_variable_name() {
    let error = ScriptError {
        info: ErrorInfo::MissingOutputVariableName(InstructionMetaInfo::new()),
    };
    println!("{}", error);
}

#[test]
fn display_invalid_equals_location() {
    let error = ScriptError {
        info: ErrorInfo::InvalidEqualsLocation(InstructionMetaInfo::new()),
    };
    println!("{}", error);
}

#[test]
fn display_invalid_quotes_location() {
    let error = ScriptError {
        info: ErrorInfo::InvalidQuotesLocation(InstructionMetaInfo::new()),
    };
    println!("{}", error);
}

#[test]
fn display_empty_label() {
    let error = ScriptError {
        info: ErrorInfo::EmptyLabel(InstructionMetaInfo::new()),
    };
    println!("{}", error);
}

#[test]
fn display_unknown_pre_processor_command() {
    let error = ScriptError {
        info: ErrorInfo::UnknownPreProcessorCommand(InstructionMetaInfo::new()),
    };
    println!("{}", error);
}
