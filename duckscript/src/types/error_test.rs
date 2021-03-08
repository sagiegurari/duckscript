use super::*;

#[test]
fn display_error_reading_file() {
    let error = ScriptError::ErrorReadingFile("test".to_string(), None);
    println!("{}", error);
}

#[test]
fn display_initialization() {
    let error = ScriptError::Initialization("test".to_string());
    println!("{}", error);
}

#[test]
fn display_runtime_with_meta_info() {
    let error = ScriptError::Runtime("test".to_string(), Some(InstructionMetaInfo::new()));
    println!("{}", error);
}

#[test]
fn display_runtime_without_meta_info() {
    let error = ScriptError::Runtime("test".to_string(), None);
    println!("{}", error);
}

#[test]
fn display_pre_process_no_command_found() {
    let error = ScriptError::PreProcessNoCommandFound(InstructionMetaInfo::new());
    println!("{}", error);
}

#[test]
fn display_control_without_valid_value() {
    let error = ScriptError::ControlWithoutValidValue(InstructionMetaInfo::new());
    println!("{}", error);
}

#[test]
fn display_invalid_control_location() {
    let error = ScriptError::InvalidControlLocation(InstructionMetaInfo::new());
    println!("{}", error);
}

#[test]
fn display_missing_end_quotes() {
    let error = ScriptError::MissingEndQuotes(InstructionMetaInfo::new());
    println!("{}", error);
}

#[test]
fn display_missing_output_variable_name() {
    let error = ScriptError::MissingOutputVariableName(InstructionMetaInfo::new());
    println!("{}", error);
}

#[test]
fn display_invalid_equals_location() {
    let error = ScriptError::InvalidEqualsLocation(InstructionMetaInfo::new());
    println!("{}", error);
}

#[test]
fn display_invalid_quotes_location() {
    let error = ScriptError::InvalidQuotesLocation(InstructionMetaInfo::new());
    println!("{}", error);
}

#[test]
fn display_empty_label() {
    let error = ScriptError::EmptyLabel(InstructionMetaInfo::new());
    println!("{}", error);
}

#[test]
fn display_unknown_pre_processor_command() {
    let error = ScriptError::UnknownPreProcessorCommand(InstructionMetaInfo::new());
    println!("{}", error);
}
