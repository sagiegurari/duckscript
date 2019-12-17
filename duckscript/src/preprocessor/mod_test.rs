use super::*;
use crate::instruction::{InstructionMetaInfo, PreProcessInstruction};

#[test]
fn run_not_preprocessor_instruction() {
    let instruction = Instruction {
        meta_info: InstructionMetaInfo::new(),
        instruction_type: InstructionType::Empty,
    };

    let output = run(&instruction);

    assert!(output.is_ok());
    assert!(output.unwrap().is_empty());
}

#[test]
fn run_no_command() {
    let instruction = Instruction {
        meta_info: InstructionMetaInfo::new(),
        instruction_type: InstructionType::PreProcess(PreProcessInstruction::new()),
    };

    let output = run(&instruction);

    assert!(output.is_err());
}

#[test]
fn run_not_supported_command() {
    let mut pre_process_instruction = PreProcessInstruction::new();
    pre_process_instruction.command = Some("test".to_string());
    let instruction = Instruction {
        meta_info: InstructionMetaInfo::new(),
        instruction_type: InstructionType::PreProcess(pre_process_instruction),
    };

    let output = run(&instruction);

    assert!(output.is_err());
}

#[test]
fn run_print_no_arguments() {
    let mut pre_process_instruction = PreProcessInstruction::new();
    pre_process_instruction.command = Some("print".to_string());
    let instruction = Instruction {
        meta_info: InstructionMetaInfo::new(),
        instruction_type: InstructionType::PreProcess(pre_process_instruction),
    };

    let output = run(&instruction);

    assert!(output.is_ok());
    assert!(output.unwrap().is_empty());
}

#[test]
fn run_print_multi_arguments() {
    let mut pre_process_instruction = PreProcessInstruction::new();
    pre_process_instruction.command = Some("print".to_string());
    pre_process_instruction.arguments = Some(vec!["arg1".to_string(), "arg2".to_string()]);
    let instruction = Instruction {
        meta_info: InstructionMetaInfo::new(),
        instruction_type: InstructionType::PreProcess(pre_process_instruction),
    };

    let output = run(&instruction);

    assert!(output.is_ok());
    assert!(output.unwrap().is_empty());
}
