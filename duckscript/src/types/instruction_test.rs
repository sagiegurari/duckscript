use super::*;

#[test]
fn pre_proces_instruction_new() {
    let value = PreProcessInstruction::new();

    assert!(value.command.is_none());
    assert!(value.arguments.is_none());
}

#[test]
fn pre_proces_instruction_is_actionable() {
    let mut value = PreProcessInstruction::new();

    assert!(!value.is_actionable());

    value.command = Some("test".to_string());

    assert!(value.is_actionable());
}

#[test]
fn script_instruction_new() {
    let value = ScriptInstruction::new();

    assert!(value.label.is_none());
    assert!(value.output.is_none());
    assert!(value.command.is_none());
    assert!(value.arguments.is_none());
}

#[test]
fn script_instruction_is_actionable() {
    let mut value = ScriptInstruction::new();

    assert!(!value.is_actionable());

    value.command = Some("test".to_string());

    assert!(value.is_actionable());
}

#[test]
fn instruction_meta_info_new() {
    let value = InstructionMetaInfo::new();

    assert!(value.line.is_none());
    assert!(value.source.is_none());
}

#[test]
fn instruction_empty_is_actionable() {
    let value = Instruction {
        meta_info: InstructionMetaInfo::new(),
        instruction_type: InstructionType::Empty,
    };

    assert!(!value.is_actionable());
}

#[test]
fn instruction_pre_process_no_command_is_actionable() {
    let pre_process_instruction = PreProcessInstruction::new();
    let value = Instruction {
        meta_info: InstructionMetaInfo::new(),
        instruction_type: InstructionType::PreProcess(pre_process_instruction),
    };

    assert!(!value.is_actionable());
}

#[test]
fn instruction_pre_process_with_command_is_actionable() {
    let mut pre_process_instruction = PreProcessInstruction::new();
    pre_process_instruction.command = Some("test".to_string());
    let value = Instruction {
        meta_info: InstructionMetaInfo::new(),
        instruction_type: InstructionType::PreProcess(pre_process_instruction),
    };

    assert!(value.is_actionable());
}

#[test]
fn instruction_script_no_command_is_actionable() {
    let script_instruction = ScriptInstruction::new();
    let value = Instruction {
        meta_info: InstructionMetaInfo::new(),
        instruction_type: InstructionType::Script(script_instruction),
    };

    assert!(!value.is_actionable());
}

#[test]
fn instruction_script_with_command_is_actionable() {
    let mut script_instruction = ScriptInstruction::new();
    script_instruction.command = Some("test".to_string());
    let value = Instruction {
        meta_info: InstructionMetaInfo::new(),
        instruction_type: InstructionType::Script(script_instruction),
    };

    assert!(value.is_actionable());
}
