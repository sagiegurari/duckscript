use super::*;
use duckscript::types::instruction::{InstructionMetaInfo, ScriptInstruction};

fn create_valid_test_if_else_block() -> Vec<Instruction> {
    let mut instructions = vec![];

    instructions.push(Instruction {
        meta_info: InstructionMetaInfo::new(),
        instruction_type: InstructionType::Empty,
    });
    let mut script_instruction = ScriptInstruction::new();
    script_instruction.output = Some("out".to_string());
    script_instruction.command = Some("test_set".to_string());
    script_instruction.arguments = Some(vec!["if".to_string()]);
    instructions.push(Instruction {
        meta_info: InstructionMetaInfo::new(),
        instruction_type: InstructionType::Script(script_instruction),
    });
    script_instruction = ScriptInstruction::new();
    script_instruction.command = Some("elseif".to_string());
    instructions.push(Instruction {
        meta_info: InstructionMetaInfo::new(),
        instruction_type: InstructionType::Script(script_instruction),
    });
    instructions.push(Instruction {
        meta_info: InstructionMetaInfo::new(),
        instruction_type: InstructionType::Empty,
    });
    script_instruction = ScriptInstruction::new();
    script_instruction.output = Some("out".to_string());
    script_instruction.command = Some("test_set".to_string());
    script_instruction.arguments = Some(vec!["elseif".to_string()]);
    instructions.push(Instruction {
        meta_info: InstructionMetaInfo::new(),
        instruction_type: InstructionType::Script(script_instruction),
    });
    script_instruction = ScriptInstruction::new();
    script_instruction.command = Some("else".to_string());
    instructions.push(Instruction {
        meta_info: InstructionMetaInfo::new(),
        instruction_type: InstructionType::Script(script_instruction),
    });
    instructions.push(Instruction {
        meta_info: InstructionMetaInfo::new(),
        instruction_type: InstructionType::Empty,
    });
    script_instruction = ScriptInstruction::new();
    script_instruction.output = Some("out".to_string());
    script_instruction.command = Some("test_set".to_string());
    script_instruction.arguments = Some(vec!["else".to_string()]);
    instructions.push(Instruction {
        meta_info: InstructionMetaInfo::new(),
        instruction_type: InstructionType::Script(script_instruction),
    });
    script_instruction = ScriptInstruction::new();
    script_instruction.command = Some("end_if".to_string());
    instructions.push(Instruction {
        meta_info: InstructionMetaInfo::new(),
        instruction_type: InstructionType::Script(script_instruction),
    });

    instructions
}

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

#[test]
fn find_commands_nested_not_allowed() {
    let mut instructions = create_valid_test_if_else_block();

    let mut script_instruction = ScriptInstruction::new();
    script_instruction.command = Some("if".to_string());
    instructions.insert(
        0,
        Instruction {
            meta_info: InstructionMetaInfo::new(),
            instruction_type: InstructionType::Script(script_instruction),
        },
    );
    script_instruction = ScriptInstruction::new();
    script_instruction.command = Some("end_if".to_string());
    instructions.insert(
        1,
        Instruction {
            meta_info: InstructionMetaInfo::new(),
            instruction_type: InstructionType::Script(script_instruction),
        },
    );

    let result = find_commands(
        &instructions,
        &vec!["if".to_string()],
        &vec!["elseif".to_string(), "else".to_string()],
        &vec!["end_if".to_string()],
        None,
        None,
        false,
    );

    assert!(result.is_err());
}

#[test]
fn find_commands_missing_end() {
    let mut instructions = create_valid_test_if_else_block();
    instructions.pop();

    let result = find_commands(
        &instructions,
        &vec!["if".to_string()],
        &vec!["elseif".to_string(), "else".to_string()],
        &vec!["end_if".to_string()],
        None,
        None,
        false,
    );

    assert!(result.is_err());
}

#[test]
fn find_commands_simple_valid() {
    let instructions = create_valid_test_if_else_block();

    let result = find_commands(
        &instructions,
        &vec!["if".to_string()],
        &vec!["elseif".to_string(), "else".to_string()],
        &vec!["end_if".to_string()],
        None,
        None,
        false,
    );

    let positions_option = result.unwrap();
    let positions = positions_option.unwrap();

    assert_eq!(positions.middle, vec![2, 5]);
    assert_eq!(positions.end, 8);
}

#[test]
fn find_commands_nested_valid() {
    let mut instructions = create_valid_test_if_else_block();
    let nested_instructions = create_valid_test_if_else_block();
    let mut script_instruction = ScriptInstruction::new();
    script_instruction.command = Some("if".to_string());
    instructions.insert(
        1,
        Instruction {
            meta_info: InstructionMetaInfo::new(),
            instruction_type: InstructionType::Script(script_instruction),
        },
    );
    let mut index = 2;
    for instruction in nested_instructions {
        instructions.insert(index, instruction);
        index = index + 1;
    }

    let result = find_commands(
        &instructions,
        &vec!["if".to_string()],
        &vec!["elseif".to_string(), "else".to_string()],
        &vec!["end_if".to_string()],
        None,
        None,
        true,
    );

    let positions_option = result.unwrap();
    let positions = positions_option.unwrap();

    assert_eq!(positions.middle, vec![12, 15]);
    assert_eq!(positions.end, 18);
}

#[test]
fn find_commands_middle_not_found() {
    let mut instructions = vec![];

    let mut script_instruction = ScriptInstruction::new();
    script_instruction.command = Some("end_if".to_string());
    instructions.push(Instruction {
        meta_info: InstructionMetaInfo::new(),
        instruction_type: InstructionType::Script(script_instruction),
    });

    let result = find_commands(
        &instructions,
        &vec!["if".to_string()],
        &vec!["elseif".to_string(), "else".to_string()],
        &vec!["end_if".to_string()],
        None,
        None,
        false,
    );

    let positions_option = result.unwrap();
    let positions = positions_option.unwrap();

    assert!(positions.middle.is_empty());
    assert_eq!(positions.end, 0);
}
