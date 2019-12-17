use crate::instruction::{Instruction, InstructionType, PreProcessInstruction, ScriptInstruction};

pub(crate) fn get_pre_process_instruction(instruction: Instruction) -> PreProcessInstruction {
    match instruction.instruction_type {
        InstructionType::PreProcess(value) => value,
        _ => panic!("Wrong instruction type."),
    }
}

pub(crate) fn get_script_instruction(instruction: Instruction) -> ScriptInstruction {
    match instruction.instruction_type {
        InstructionType::Script(value) => value,
        _ => panic!("Wrong instruction type."),
    }
}

pub(crate) fn assert_empty_instruction(instruction: Instruction) {
    match instruction.instruction_type {
        InstructionType::Empty => (),
        _ => panic!("Wrong instruction type."),
    };
}
