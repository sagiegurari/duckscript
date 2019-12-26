use super::*;
use duckscript::types::instruction::{InstructionMetaInfo, ScriptInstruction};

#[test]
fn find_command_empty() {
    let result = find_command(&vec![], &vec![], None, None, &vec![]);

    assert!(result.is_err());
}

#[test]
fn find_command_no_instructions() {
    let result = find_command(&vec![], &vec!["1".to_string()], None, None, &vec![]);

    assert!(result.is_ok());
    assert!(result.unwrap().is_none());
}

#[test]
fn find_command_only_empty_instructions() {
    let result = find_command(
        &vec![
            Instruction {
                meta_info: InstructionMetaInfo::new(),
                instruction_type: InstructionType::Empty,
            },
            Instruction {
                meta_info: InstructionMetaInfo::new(),
                instruction_type: InstructionType::Empty,
            },
            Instruction {
                meta_info: InstructionMetaInfo::new(),
                instruction_type: InstructionType::Empty,
            },
        ],
        &vec!["1".to_string()],
        None,
        None,
        &vec![],
    );

    assert!(result.is_ok());
    assert!(result.unwrap().is_none());
}

#[test]
fn find_command_not_found() {
    let mut instructions = vec![];

    let mut script_instruction = ScriptInstruction::new();
    script_instruction.command = Some("a".to_string());
    instructions.push(Instruction {
        meta_info: InstructionMetaInfo::new(),
        instruction_type: InstructionType::Script(script_instruction),
    });
    script_instruction = ScriptInstruction::new();
    script_instruction.command = Some("b".to_string());
    instructions.push(Instruction {
        meta_info: InstructionMetaInfo::new(),
        instruction_type: InstructionType::Script(script_instruction),
    });
    script_instruction = ScriptInstruction::new();
    script_instruction.command = Some("c".to_string());
    instructions.push(Instruction {
        meta_info: InstructionMetaInfo::new(),
        instruction_type: InstructionType::Script(script_instruction),
    });

    let result = find_command(&instructions, &vec!["1".to_string()], None, None, &vec![]);

    assert!(result.is_ok());
    assert!(result.unwrap().is_none());
}

#[test]
fn find_command_found() {
    let mut instructions = vec![];

    let mut script_instruction = ScriptInstruction::new();
    script_instruction.command = Some("a".to_string());
    instructions.push(Instruction {
        meta_info: InstructionMetaInfo::new(),
        instruction_type: InstructionType::Script(script_instruction),
    });
    script_instruction = ScriptInstruction::new();
    script_instruction.command = Some("b".to_string());
    instructions.push(Instruction {
        meta_info: InstructionMetaInfo::new(),
        instruction_type: InstructionType::Script(script_instruction),
    });
    script_instruction = ScriptInstruction::new();
    script_instruction.command = Some("c".to_string());
    instructions.push(Instruction {
        meta_info: InstructionMetaInfo::new(),
        instruction_type: InstructionType::Script(script_instruction),
    });
    script_instruction = ScriptInstruction::new();
    script_instruction.command = Some("1".to_string());
    instructions.push(Instruction {
        meta_info: InstructionMetaInfo::new(),
        instruction_type: InstructionType::Script(script_instruction),
    });

    let result = find_command(&instructions, &vec!["1".to_string()], None, None, &vec![]);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().unwrap(), 3);
}

#[test]
fn find_command_found_after_start_index() {
    let mut instructions = vec![];

    let mut script_instruction = ScriptInstruction::new();
    script_instruction.command = Some("a".to_string());
    instructions.push(Instruction {
        meta_info: InstructionMetaInfo::new(),
        instruction_type: InstructionType::Script(script_instruction),
    });
    script_instruction = ScriptInstruction::new();
    script_instruction.command = Some("1".to_string());
    instructions.push(Instruction {
        meta_info: InstructionMetaInfo::new(),
        instruction_type: InstructionType::Script(script_instruction),
    });
    script_instruction = ScriptInstruction::new();
    script_instruction.command = Some("b".to_string());
    instructions.push(Instruction {
        meta_info: InstructionMetaInfo::new(),
        instruction_type: InstructionType::Script(script_instruction),
    });
    script_instruction = ScriptInstruction::new();
    script_instruction.command = Some("c".to_string());
    instructions.push(Instruction {
        meta_info: InstructionMetaInfo::new(),
        instruction_type: InstructionType::Script(script_instruction),
    });

    let result = find_command(
        &instructions,
        &vec!["1".to_string()],
        Some(2),
        None,
        &vec![],
    );

    assert!(result.is_ok());
    assert!(result.unwrap().is_none());
}

#[test]
fn find_command_found_with_start_index() {
    let mut instructions = vec![];

    let mut script_instruction = ScriptInstruction::new();
    script_instruction.command = Some("a".to_string());
    instructions.push(Instruction {
        meta_info: InstructionMetaInfo::new(),
        instruction_type: InstructionType::Script(script_instruction),
    });
    script_instruction = ScriptInstruction::new();
    script_instruction.command = Some("b".to_string());
    instructions.push(Instruction {
        meta_info: InstructionMetaInfo::new(),
        instruction_type: InstructionType::Script(script_instruction),
    });
    script_instruction = ScriptInstruction::new();
    script_instruction.command = Some("1".to_string());
    instructions.push(Instruction {
        meta_info: InstructionMetaInfo::new(),
        instruction_type: InstructionType::Script(script_instruction),
    });
    script_instruction = ScriptInstruction::new();
    script_instruction.command = Some("c".to_string());
    instructions.push(Instruction {
        meta_info: InstructionMetaInfo::new(),
        instruction_type: InstructionType::Script(script_instruction),
    });

    let result = find_command(
        &instructions,
        &vec!["1".to_string()],
        Some(2),
        None,
        &vec![],
    );

    assert!(result.is_ok());
    assert_eq!(result.unwrap().unwrap(), 2);
}

#[test]
fn find_command_found_before_end_index() {
    let mut instructions = vec![];

    let mut script_instruction = ScriptInstruction::new();
    script_instruction.command = Some("a".to_string());
    instructions.push(Instruction {
        meta_info: InstructionMetaInfo::new(),
        instruction_type: InstructionType::Script(script_instruction),
    });
    script_instruction = ScriptInstruction::new();
    script_instruction.command = Some("b".to_string());
    instructions.push(Instruction {
        meta_info: InstructionMetaInfo::new(),
        instruction_type: InstructionType::Script(script_instruction),
    });
    script_instruction = ScriptInstruction::new();
    script_instruction.command = Some("c".to_string());
    instructions.push(Instruction {
        meta_info: InstructionMetaInfo::new(),
        instruction_type: InstructionType::Script(script_instruction),
    });
    script_instruction = ScriptInstruction::new();
    script_instruction.command = Some("1".to_string());
    instructions.push(Instruction {
        meta_info: InstructionMetaInfo::new(),
        instruction_type: InstructionType::Script(script_instruction),
    });

    let result = find_command(
        &instructions,
        &vec!["1".to_string()],
        None,
        Some(2),
        &vec![],
    );

    assert!(result.is_ok());
    assert!(result.unwrap().is_none());
}

#[test]
fn find_command_found_after_error() {
    let mut instructions = vec![];

    let mut script_instruction = ScriptInstruction::new();
    script_instruction.command = Some("a".to_string());
    instructions.push(Instruction {
        meta_info: InstructionMetaInfo::new(),
        instruction_type: InstructionType::Script(script_instruction),
    });
    script_instruction = ScriptInstruction::new();
    script_instruction.command = Some("b".to_string());
    instructions.push(Instruction {
        meta_info: InstructionMetaInfo::new(),
        instruction_type: InstructionType::Script(script_instruction),
    });
    script_instruction = ScriptInstruction::new();
    script_instruction.command = Some("c".to_string());
    instructions.push(Instruction {
        meta_info: InstructionMetaInfo::new(),
        instruction_type: InstructionType::Script(script_instruction),
    });
    script_instruction = ScriptInstruction::new();
    script_instruction.command = Some("1".to_string());
    instructions.push(Instruction {
        meta_info: InstructionMetaInfo::new(),
        instruction_type: InstructionType::Script(script_instruction),
    });

    let result = find_command(
        &instructions,
        &vec!["1".to_string()],
        None,
        None,
        &vec!["b".to_string()],
    );

    assert!(result.is_err());
}

#[test]
fn find_command_found_before_error() {
    let mut instructions = vec![];

    let mut script_instruction = ScriptInstruction::new();
    script_instruction.command = Some("a".to_string());
    instructions.push(Instruction {
        meta_info: InstructionMetaInfo::new(),
        instruction_type: InstructionType::Script(script_instruction),
    });
    script_instruction = ScriptInstruction::new();
    script_instruction.command = Some("1".to_string());
    instructions.push(Instruction {
        meta_info: InstructionMetaInfo::new(),
        instruction_type: InstructionType::Script(script_instruction),
    });
    script_instruction = ScriptInstruction::new();
    script_instruction.command = Some("b".to_string());
    instructions.push(Instruction {
        meta_info: InstructionMetaInfo::new(),
        instruction_type: InstructionType::Script(script_instruction),
    });
    script_instruction = ScriptInstruction::new();
    script_instruction.command = Some("c".to_string());
    instructions.push(Instruction {
        meta_info: InstructionMetaInfo::new(),
        instruction_type: InstructionType::Script(script_instruction),
    });

    let result = find_command(
        &instructions,
        &vec!["1".to_string()],
        None,
        None,
        &vec!["b".to_string()],
    );

    assert!(result.is_ok());
    assert_eq!(result.unwrap().unwrap(), 1);
}
