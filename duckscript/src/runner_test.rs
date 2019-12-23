use super::*;

use crate::test::{ErrorCommand, ExitCommand};
use crate::types::instruction::{InstructionMetaInfo, PreProcessInstruction};

#[test]
fn run_script_parse_error() {
    let result = run_script("!bad", Context::new());

    assert!(result.is_err());
}

#[test]
fn run_script_valid() {
    let result = run_script("!print test", Context::new());

    assert!(result.is_ok());
}

#[test]
fn run_script_runtime_error() {
    let result = run_script_file("./src/test/scripts/echo.ds", Context::new());

    assert!(result.is_err());
}

#[test]
fn run_script_file_valid() {
    let result = run_script_file("./src/test/scripts/print_preprocessor.ds", Context::new());

    assert!(result.is_ok());
}

#[test]
fn run_no_instructions() {
    let result = run(vec![], Context::new());

    assert!(result.is_ok());
}

#[test]
fn run_empty_instructions() {
    let mut instructions = vec![];
    instructions.push(Instruction {
        meta_info: InstructionMetaInfo::new(),
        instruction_type: InstructionType::Empty,
    });
    instructions.push(Instruction {
        meta_info: InstructionMetaInfo::new(),
        instruction_type: InstructionType::Empty,
    });

    let result = run(instructions, Context::new());

    assert!(result.is_ok());
}

#[test]
fn run_pre_processor_instructions() {
    let mut instructions = vec![];
    instructions.push(Instruction {
        meta_info: InstructionMetaInfo::new(),
        instruction_type: InstructionType::PreProcess(PreProcessInstruction::new()),
    });
    instructions.push(Instruction {
        meta_info: InstructionMetaInfo::new(),
        instruction_type: InstructionType::PreProcess(PreProcessInstruction::new()),
    });

    let result = run(instructions, Context::new());

    assert!(result.is_ok());
}

#[test]
fn run_no_command_script_instructions() {
    let mut instructions = vec![];
    instructions.push(Instruction {
        meta_info: InstructionMetaInfo::new(),
        instruction_type: InstructionType::Script(ScriptInstruction::new()),
    });
    instructions.push(Instruction {
        meta_info: InstructionMetaInfo::new(),
        instruction_type: InstructionType::Script(ScriptInstruction::new()),
    });

    let result = run(instructions, Context::new());

    assert!(result.is_ok());
}

#[test]
fn run_all_types_instructions() {
    let mut instructions = vec![];
    instructions.push(Instruction {
        meta_info: InstructionMetaInfo::new(),
        instruction_type: InstructionType::Empty,
    });
    instructions.push(Instruction {
        meta_info: InstructionMetaInfo::new(),
        instruction_type: InstructionType::PreProcess(PreProcessInstruction::new()),
    });
    instructions.push(Instruction {
        meta_info: InstructionMetaInfo::new(),
        instruction_type: InstructionType::Script(ScriptInstruction::new()),
    });

    let result = run(instructions, Context::new());

    assert!(result.is_ok());
}

#[test]
fn create_runtime_with_labels() {
    let mut instructions = vec![];

    let mut script_instruction = ScriptInstruction::new();
    script_instruction.label = Some("test1".to_string());
    instructions.push(Instruction {
        meta_info: InstructionMetaInfo::new(),
        instruction_type: InstructionType::Script(script_instruction),
    });
    script_instruction = ScriptInstruction::new();
    script_instruction.label = Some("test2".to_string());
    instructions.push(Instruction {
        meta_info: InstructionMetaInfo::new(),
        instruction_type: InstructionType::Script(script_instruction),
    });

    let runtime = create_runtime(instructions, Context::new());

    assert_eq!(runtime.label_to_line.get("test1"), Some(&0));
    assert_eq!(runtime.label_to_line.get("test2"), Some(&1));
}

#[test]
fn run_instructions_unknown_command() {
    let mut instructions = vec![];

    let mut script_instruction = ScriptInstruction::new();
    script_instruction.command = Some("bad".to_string());
    instructions.push(Instruction {
        meta_info: InstructionMetaInfo::new(),
        instruction_type: InstructionType::Script(script_instruction),
    });

    let runtime = create_runtime(instructions, Context::new());

    let context_result = run_instructions(runtime, 0);

    assert!(context_result.is_err());
}

#[test]
fn run_instructions_start_bigger_then_script() {
    let mut instructions = vec![];

    instructions.push(Instruction {
        meta_info: InstructionMetaInfo::new(),
        instruction_type: InstructionType::Script(ScriptInstruction::new()),
    });

    let runtime = create_runtime(instructions, Context::new());

    let context_result = run_instructions(runtime, 10);

    assert!(context_result.is_ok());
}

#[test]
fn run_instructions_start_after_exit() {
    let mut instructions = vec![];

    let mut script_instruction = ScriptInstruction::new();
    script_instruction.command = Some("exit".to_string());
    instructions.push(Instruction {
        meta_info: InstructionMetaInfo::new(),
        instruction_type: InstructionType::Script(script_instruction),
    });
    script_instruction = ScriptInstruction::new();
    script_instruction.command = Some("bad".to_string());
    instructions.push(Instruction {
        meta_info: InstructionMetaInfo::new(),
        instruction_type: InstructionType::Script(script_instruction),
    });

    let mut context = Context::new();
    let result = context.commands.set(Box::new(ExitCommand {}));
    assert!(result.is_ok());

    let runtime = create_runtime(instructions, context);

    let context_result = run_instructions(runtime, 1);

    assert!(context_result.is_err());
}

#[test]
fn run_instructions_exit_result_no_output() {
    let mut instructions = vec![];

    let mut script_instruction = ScriptInstruction::new();
    script_instruction.command = Some("exit".to_string());
    instructions.push(Instruction {
        meta_info: InstructionMetaInfo::new(),
        instruction_type: InstructionType::Script(script_instruction),
    });
    script_instruction = ScriptInstruction::new();
    script_instruction.command = Some("bad".to_string());
    instructions.push(Instruction {
        meta_info: InstructionMetaInfo::new(),
        instruction_type: InstructionType::Script(script_instruction),
    });

    let mut context = Context::new();
    let result = context.commands.set(Box::new(ExitCommand {}));
    assert!(result.is_ok());

    let runtime = create_runtime(instructions, context);

    let context_result = run_instructions(runtime, 0);

    assert!(context_result.is_ok());
    assert!(context_result.unwrap().variables.is_empty());
}

#[test]
fn run_instructions_exit_result_with_output() {
    let mut instructions = vec![];

    let mut script_instruction = ScriptInstruction::new();
    script_instruction.command = Some("exit".to_string());
    script_instruction.output = Some("out".to_string());
    script_instruction.arguments = Some(vec!["value".to_string()]);
    instructions.push(Instruction {
        meta_info: InstructionMetaInfo::new(),
        instruction_type: InstructionType::Script(script_instruction),
    });
    script_instruction = ScriptInstruction::new();
    script_instruction.command = Some("bad".to_string());
    instructions.push(Instruction {
        meta_info: InstructionMetaInfo::new(),
        instruction_type: InstructionType::Script(script_instruction),
    });

    let mut context = Context::new();
    let result = context.commands.set(Box::new(ExitCommand {}));
    assert!(result.is_ok());

    let runtime = create_runtime(instructions, context);

    let context_result = run_instructions(runtime, 0);

    assert!(context_result.is_ok());
    assert_eq!(
        context_result.unwrap().variables.get("out"),
        Some(&"value".to_string())
    );
}

#[test]
fn run_instructions_error_result() {
    let mut instructions = vec![];

    let mut script_instruction = ScriptInstruction::new();
    script_instruction.command = Some("error".to_string());
    instructions.push(Instruction {
        meta_info: InstructionMetaInfo::new(),
        instruction_type: InstructionType::Script(script_instruction),
    });

    let mut context = Context::new();
    let result = context.commands.set(Box::new(ErrorCommand {}));
    assert!(result.is_ok());

    let runtime = create_runtime(instructions, context);

    let context_result = run_instructions(runtime, 0);

    assert!(context_result.is_err());
}
