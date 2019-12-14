use crate::instruction::{Instruction, InstructionType, PreProcessInstruction};

pub(crate) fn get_pre_process_instruction(instruction: Instruction) -> PreProcessInstruction {
    match instruction.instruction_type {
        InstructionType::PreProcess(value) => value,
        _ => panic!("Wrong instruction type."),
    }
}
