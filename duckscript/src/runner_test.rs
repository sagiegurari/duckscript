use super::*;

use crate::test;
use crate::test::{
    CrashCommand, ErrorCommand, ExitCommand, GoToLabelCommand, GoToLineCommand, OnErrorCommand,
    SetCommand,
};
use crate::types::instruction::PreProcessInstruction;

fn assert_end_reason_exit_called(end_reason: EndReason) {
    match end_reason {
        EndReason::ExitCalled => (),
        _ => panic!("Invalid end reason."),
    }
}

fn assert_end_reason_reached_end(end_reason: EndReason) {
    match end_reason {
        EndReason::ReachedEnd => (),
        _ => panic!("Invalid end reason."),
    }
}

#[test]
fn run_script_parse_error() {
    let result = run_script("!bad", Context::new(), None);

    assert!(result.is_err());
}

#[test]
fn run_script_valid() {
    let result = run_script("!print test", Context::new(), None);

    assert!(result.is_ok());
}

#[test]
fn run_script_runtime_error() {
    let mut context = Context::new();
    let cmd_result = context.commands.set(Box::new(CrashCommand {}));
    assert!(cmd_result.is_ok());
    let result = run_script_file("./src/test/scripts/crash.ds", context, None);

    assert!(result.is_err());
}

#[test]
fn run_script_file_valid() {
    let result = run_script_file(
        "./src/test/scripts/print_preprocessor.ds",
        Context::new(),
        None,
    );

    assert!(result.is_ok());
}

#[test]
fn run_no_instructions() {
    let result = run(vec![], Context::new(), None);

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

    let result = run(instructions, Context::new(), None);

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

    let result = run(instructions, Context::new(), None);

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

    let result = run(instructions, Context::new(), None);

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

    let result = run(instructions, Context::new(), None);

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

    let runtime = create_runtime(instructions, Context::new(), None);

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

    let runtime = create_runtime(instructions, Context::new(), None);

    let context_result = run_instructions(runtime, 0, false);

    assert!(context_result.is_err());
}

#[test]
fn run_instructions_start_bigger_then_script() {
    let mut instructions = vec![];

    instructions.push(Instruction {
        meta_info: InstructionMetaInfo::new(),
        instruction_type: InstructionType::Script(ScriptInstruction::new()),
    });

    let runtime = create_runtime(instructions, Context::new(), None);

    let context_result = run_instructions(runtime, 10, false);

    assert!(context_result.is_ok());
}

#[test]
fn run_instructions_start_after_bad_command() {
    let mut instructions = vec![];

    let mut script_instruction = ScriptInstruction::new();
    script_instruction.command = Some("bad".to_string());
    instructions.push(Instruction {
        meta_info: InstructionMetaInfo::new(),
        instruction_type: InstructionType::Script(script_instruction),
    });
    script_instruction = ScriptInstruction::new();
    script_instruction.command = Some("set".to_string());
    instructions.push(Instruction {
        meta_info: InstructionMetaInfo::new(),
        instruction_type: InstructionType::Script(script_instruction),
    });

    let mut context = Context::new();
    let mut result = context.commands.set(Box::new(ExitCommand {}));
    assert!(result.is_ok());
    result = context.commands.set(Box::new(SetCommand {}));
    assert!(result.is_ok());

    let runtime = create_runtime(instructions, context, None);

    let context_result = run_instructions(runtime, 1, false);

    assert!(context_result.is_ok());
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

    let runtime = create_runtime(instructions, context, None);

    let context_result = run_instructions(runtime, 0, false);

    assert!(context_result.is_ok());
    let (updated_context, end_reason) = context_result.unwrap();
    assert!(updated_context.variables.is_empty());
    assert_end_reason_exit_called(end_reason);
}

#[test]
fn run_instructions_exit_result_with_string_output() {
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

    let runtime = create_runtime(instructions, context, None);

    let context_result = run_instructions(runtime, 0, false);

    assert!(context_result.is_ok());
    let (updated_context, end_reason) = context_result.unwrap();
    assert_end_reason_exit_called(end_reason);
    assert_eq!(
        updated_context.variables.get("out"),
        Some(&"value".to_string())
    );
}

#[test]
fn run_instructions_exit_result_with_0_output() {
    let mut instructions = vec![];

    let mut script_instruction = ScriptInstruction::new();
    script_instruction.command = Some("exit".to_string());
    script_instruction.output = Some("out".to_string());
    script_instruction.arguments = Some(vec!["0".to_string()]);
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

    let runtime = create_runtime(instructions, context, None);

    let context_result = run_instructions(runtime, 0, false);

    assert!(context_result.is_ok());
    let (updated_context, end_reason) = context_result.unwrap();
    assert_end_reason_exit_called(end_reason);
    assert_eq!(updated_context.variables.get("out"), Some(&"0".to_string()));
}

#[test]
fn run_instructions_exit_result_with_error_code_output() {
    let mut instructions = vec![];

    let mut script_instruction = ScriptInstruction::new();
    script_instruction.command = Some("exit".to_string());
    script_instruction.output = Some("out".to_string());
    script_instruction.arguments = Some(vec!["1".to_string()]);
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

    let runtime = create_runtime(instructions, context, None);

    let context_result = run_instructions(runtime, 0, false);

    assert!(context_result.is_err());
}

#[test]
fn run_instructions_error_result() {
    let mut instructions = vec![];

    let mut script_instruction = ScriptInstruction::new();
    script_instruction.output = Some("out".to_string());
    script_instruction.command = Some("error".to_string());
    instructions.push(Instruction {
        meta_info: InstructionMetaInfo::new(),
        instruction_type: InstructionType::Script(script_instruction),
    });

    let mut context = Context::new();
    let result = context.commands.set(Box::new(ErrorCommand {}));
    assert!(result.is_ok());

    let runtime = create_runtime(instructions, context, None);

    let context_result = run_instructions(runtime, 0, false);

    assert!(context_result.is_ok());
    let (updated_context, end_reason) = context_result.unwrap();
    assert_end_reason_reached_end(end_reason);
    let variables = updated_context.variables;
    assert!(!variables.is_empty());
    assert_eq!(variables.get("out").unwrap(), "false");
}

#[test]
fn run_instructions_error_result_with_on_error() {
    let mut instructions = vec![];

    instructions.push(Instruction {
        meta_info: InstructionMetaInfo::new(),
        instruction_type: InstructionType::Empty,
    });
    instructions.push(Instruction {
        meta_info: InstructionMetaInfo::new(),
        instruction_type: InstructionType::Empty,
    });

    let mut meta_info = InstructionMetaInfo::new();
    meta_info.line = Some(2);
    meta_info.source = Some("myfile".to_string());
    let mut script_instruction = ScriptInstruction::new();
    script_instruction.command = Some("error".to_string());
    instructions.push(Instruction {
        meta_info,
        instruction_type: InstructionType::Script(script_instruction),
    });

    let mut context = Context::new();
    let mut result = context.commands.set(Box::new(ErrorCommand {}));
    assert!(result.is_ok());
    result = context.commands.set(Box::new(OnErrorCommand {}));
    assert!(result.is_ok());

    let runtime = create_runtime(instructions, context, None);

    let context_result = run_instructions(runtime, 2, false);

    assert!(context_result.is_ok());

    let (updated_context, end_reason) = context_result.unwrap();
    assert_end_reason_reached_end(end_reason);
    let variables = updated_context.variables;
    assert!(!variables.is_empty());
    assert_eq!(variables.get("1").unwrap(), "test");
    assert_eq!(variables.get("2").unwrap(), "2");
    assert_eq!(variables.get("3").unwrap(), "myfile");
}

#[test]
fn run_instructions_crash_result() {
    let mut instructions = vec![];

    let mut script_instruction = ScriptInstruction::new();
    script_instruction.command = Some("crash".to_string());
    instructions.push(Instruction {
        meta_info: InstructionMetaInfo::new(),
        instruction_type: InstructionType::Script(script_instruction),
    });

    let mut context = Context::new();
    let result = context.commands.set(Box::new(CrashCommand {}));
    assert!(result.is_ok());

    let runtime = create_runtime(instructions, context, None);

    let context_result = run_instructions(runtime, 0, false);

    assert!(context_result.is_err());
}

#[test]
fn run_instructions_crash_result_repl_mode() {
    let mut instructions = vec![];

    let mut script_instruction = ScriptInstruction::new();
    script_instruction.command = Some("crash".to_string());
    instructions.push(Instruction {
        meta_info: InstructionMetaInfo::new(),
        instruction_type: InstructionType::Script(script_instruction),
    });

    let mut context = Context::new();
    let result = context.commands.set(Box::new(CrashCommand {}));
    assert!(result.is_ok());

    let runtime = create_runtime(instructions, context, None);

    let context_result = run_instructions(runtime, 0, true);

    assert!(context_result.is_ok());
}

#[test]
fn run_instructions_continue_result_no_output() {
    let mut instructions = vec![];

    instructions.push(Instruction {
        meta_info: InstructionMetaInfo::new(),
        instruction_type: InstructionType::Empty,
    });
    instructions.push(Instruction {
        meta_info: InstructionMetaInfo::new(),
        instruction_type: InstructionType::Empty,
    });

    let runtime = create_runtime(instructions, Context::new(), None);

    let context_result = run_instructions(runtime, 0, false);

    assert!(context_result.is_ok());
    let (updated_context, end_reason) = context_result.unwrap();
    assert_end_reason_reached_end(end_reason);
    assert!(updated_context.variables.is_empty());
}

#[test]
fn run_instructions_continue_result_with_output() {
    let mut instructions = vec![];

    let mut script_instruction = ScriptInstruction::new();
    script_instruction.command = Some("set".to_string());
    script_instruction.output = Some("out1".to_string());
    script_instruction.arguments = Some(vec!["value1".to_string()]);
    instructions.push(Instruction {
        meta_info: InstructionMetaInfo::new(),
        instruction_type: InstructionType::Script(script_instruction),
    });
    script_instruction = ScriptInstruction::new();
    script_instruction.command = Some("set".to_string());
    script_instruction.output = Some("out2".to_string());
    script_instruction.arguments = Some(vec!["value2".to_string()]);
    instructions.push(Instruction {
        meta_info: InstructionMetaInfo::new(),
        instruction_type: InstructionType::Script(script_instruction),
    });

    let mut context = Context::new();
    let result = context.commands.set(Box::new(SetCommand {}));
    assert!(result.is_ok());

    let runtime = create_runtime(instructions, context, None);

    let context_result = run_instructions(runtime, 0, false);

    assert!(context_result.is_ok());

    let (updated_context, end_reason) = context_result.unwrap();
    assert_end_reason_reached_end(end_reason);
    assert_eq!(
        updated_context.variables.get("out1"),
        Some(&"value1".to_string())
    );
    assert_eq!(
        updated_context.variables.get("out2"),
        Some(&"value2".to_string())
    );
}

#[test]
fn run_instructions_goto_label_result_no_output() {
    let mut instructions = vec![];

    let mut script_instruction = ScriptInstruction::new();
    script_instruction.command = Some("goto".to_string());
    script_instruction.arguments = Some(vec!["my_label".to_string()]);
    instructions.push(Instruction {
        meta_info: InstructionMetaInfo::new(),
        instruction_type: InstructionType::Script(script_instruction),
    });
    script_instruction = ScriptInstruction::new();
    script_instruction.command = Some("exit".to_string());
    instructions.push(Instruction {
        meta_info: InstructionMetaInfo::new(),
        instruction_type: InstructionType::Script(script_instruction),
    });
    script_instruction = ScriptInstruction::new();
    script_instruction.command = Some("set".to_string());
    script_instruction.label = Some("my_label".to_string());
    script_instruction.output = Some("out2".to_string());
    script_instruction.arguments = Some(vec!["value2".to_string()]);
    instructions.push(Instruction {
        meta_info: InstructionMetaInfo::new(),
        instruction_type: InstructionType::Script(script_instruction),
    });

    let mut context = Context::new();
    let mut result = context.commands.set(Box::new(SetCommand {}));
    assert!(result.is_ok());
    result = context.commands.set(Box::new(ExitCommand {}));
    assert!(result.is_ok());
    result = context.commands.set(Box::new(GoToLabelCommand {}));
    assert!(result.is_ok());

    let runtime = create_runtime(instructions, context, None);

    let context_result = run_instructions(runtime, 0, false);

    assert!(context_result.is_ok());

    let (updated_context, end_reason) = context_result.unwrap();
    assert_end_reason_reached_end(end_reason);
    assert_eq!(
        updated_context.variables.get("out2"),
        Some(&"value2".to_string())
    );
}

#[test]
fn run_instructions_goto_label_result_with_output() {
    let mut instructions = vec![];

    let mut script_instruction = ScriptInstruction::new();
    script_instruction.command = Some("goto".to_string());
    script_instruction.output = Some("out1".to_string());
    script_instruction.arguments = Some(vec!["my_label".to_string()]);
    instructions.push(Instruction {
        meta_info: InstructionMetaInfo::new(),
        instruction_type: InstructionType::Script(script_instruction),
    });
    script_instruction = ScriptInstruction::new();
    script_instruction.command = Some("exit".to_string());
    instructions.push(Instruction {
        meta_info: InstructionMetaInfo::new(),
        instruction_type: InstructionType::Script(script_instruction),
    });
    script_instruction = ScriptInstruction::new();
    script_instruction.command = Some("set".to_string());
    script_instruction.label = Some("my_label".to_string());
    script_instruction.output = Some("out2".to_string());
    script_instruction.arguments = Some(vec!["value2".to_string()]);
    instructions.push(Instruction {
        meta_info: InstructionMetaInfo::new(),
        instruction_type: InstructionType::Script(script_instruction),
    });

    let mut context = Context::new();
    let mut result = context.commands.set(Box::new(SetCommand {}));
    assert!(result.is_ok());
    result = context.commands.set(Box::new(ExitCommand {}));
    assert!(result.is_ok());
    result = context.commands.set(Box::new(GoToLabelCommand {}));
    assert!(result.is_ok());

    let runtime = create_runtime(instructions, context, None);

    let context_result = run_instructions(runtime, 0, false);

    assert!(context_result.is_ok());

    let (updated_context, end_reason) = context_result.unwrap();
    assert_end_reason_reached_end(end_reason);
    assert_eq!(
        updated_context.variables.get("out1"),
        Some(&"my_label".to_string())
    );
    assert_eq!(
        updated_context.variables.get("out2"),
        Some(&"value2".to_string())
    );
}

#[test]
fn run_instructions_goto_line_result_no_output() {
    let mut instructions = vec![];

    let mut script_instruction = ScriptInstruction::new();
    script_instruction.command = Some("goto".to_string());
    script_instruction.arguments = Some(vec!["2".to_string()]);
    instructions.push(Instruction {
        meta_info: InstructionMetaInfo::new(),
        instruction_type: InstructionType::Script(script_instruction),
    });
    script_instruction = ScriptInstruction::new();
    script_instruction.command = Some("exit".to_string());
    instructions.push(Instruction {
        meta_info: InstructionMetaInfo::new(),
        instruction_type: InstructionType::Script(script_instruction),
    });
    script_instruction = ScriptInstruction::new();
    script_instruction.command = Some("set".to_string());
    script_instruction.label = Some("my_label".to_string());
    script_instruction.output = Some("out2".to_string());
    script_instruction.arguments = Some(vec!["value2".to_string()]);
    instructions.push(Instruction {
        meta_info: InstructionMetaInfo::new(),
        instruction_type: InstructionType::Script(script_instruction),
    });

    let mut context = Context::new();
    let mut result = context.commands.set(Box::new(SetCommand {}));
    assert!(result.is_ok());
    result = context.commands.set(Box::new(ExitCommand {}));
    assert!(result.is_ok());
    result = context.commands.set(Box::new(GoToLineCommand {}));
    assert!(result.is_ok());

    let runtime = create_runtime(instructions, context, None);

    let context_result = run_instructions(runtime, 0, false);

    assert!(context_result.is_ok());

    let (updated_context, end_reason) = context_result.unwrap();
    assert_end_reason_reached_end(end_reason);
    assert_eq!(
        updated_context.variables.get("out2"),
        Some(&"value2".to_string())
    );
}

#[test]
fn run_instructions_goto_line_result_with_output() {
    let mut instructions = vec![];

    let mut script_instruction = ScriptInstruction::new();
    script_instruction.command = Some("goto".to_string());
    script_instruction.output = Some("out1".to_string());
    script_instruction.arguments = Some(vec!["2".to_string()]);
    instructions.push(Instruction {
        meta_info: InstructionMetaInfo::new(),
        instruction_type: InstructionType::Script(script_instruction),
    });
    script_instruction = ScriptInstruction::new();
    script_instruction.command = Some("exit".to_string());
    instructions.push(Instruction {
        meta_info: InstructionMetaInfo::new(),
        instruction_type: InstructionType::Script(script_instruction),
    });
    script_instruction = ScriptInstruction::new();
    script_instruction.command = Some("set".to_string());
    script_instruction.label = Some("my_label".to_string());
    script_instruction.output = Some("out2".to_string());
    script_instruction.arguments = Some(vec!["value2".to_string()]);
    instructions.push(Instruction {
        meta_info: InstructionMetaInfo::new(),
        instruction_type: InstructionType::Script(script_instruction),
    });

    let mut context = Context::new();
    let mut result = context.commands.set(Box::new(SetCommand {}));
    assert!(result.is_ok());
    result = context.commands.set(Box::new(ExitCommand {}));
    assert!(result.is_ok());
    result = context.commands.set(Box::new(GoToLineCommand {}));
    assert!(result.is_ok());

    let runtime = create_runtime(instructions, context, None);

    let context_result = run_instructions(runtime, 0, false);

    assert!(context_result.is_ok());

    let (updated_context, end_reason) = context_result.unwrap();
    assert_end_reason_reached_end(end_reason);
    assert_eq!(
        updated_context.variables.get("out1"),
        Some(&"2".to_string())
    );
    assert_eq!(
        updated_context.variables.get("out2"),
        Some(&"value2".to_string())
    );
}

#[test]
fn bind_command_arguments_mixed() {
    let mut context = Context::new();
    context
        .variables
        .insert("key1".to_string(), "value1".to_string());
    context
        .variables
        .insert("key2".to_string(), "value2".to_string());
    context
        .variables
        .insert("key3".to_string(), "value3".to_string());

    let mut script_instruction = ScriptInstruction::new();
    script_instruction.arguments = Some(vec![
        "${key1}".to_string(),
        "key2: ${key2}".to_string(),
        "key3: ${key3} <- key3".to_string(),
        "${key1} ${key2}".to_string(),
        "${bad} is bad".to_string(),
    ]);

    let arguments = bind_command_arguments(
        &context.variables,
        &script_instruction,
        &InstructionMetaInfo::new(),
    );

    assert_eq!(
        arguments,
        vec![
            "value1",
            "key2: value2",
            "key3: value3 <- key3",
            "value1 value2",
            " is bad"
        ]
    );
}

#[test]
fn run_instruction_empty_instruction() {
    let instruction = Instruction {
        meta_info: InstructionMetaInfo::new(),
        instruction_type: InstructionType::Empty,
    };

    let mut context = Context::new();
    let mut env = Env::default();

    let (command_result, output_variable) = run_instruction(
        &mut context.commands,
        &mut context.variables,
        &mut HashMap::new(),
        &vec![],
        instruction,
        0,
        &mut env,
    );

    assert!(output_variable.is_none());
    assert!(test::validate_continue_result(&command_result, None));
}

#[test]
fn run_instruction_pre_processor_instruction() {
    let instruction = Instruction {
        meta_info: InstructionMetaInfo::new(),
        instruction_type: InstructionType::PreProcess(PreProcessInstruction::new()),
    };

    let mut context = Context::new();
    let mut env = Env::default();

    let (command_result, output_variable) = run_instruction(
        &mut context.commands,
        &mut context.variables,
        &mut HashMap::new(),
        &vec![],
        instruction,
        0,
        &mut env,
    );

    assert!(output_variable.is_none());
    assert!(test::validate_continue_result(&command_result, None));
}

#[test]
fn run_instruction_script_instruction_no_command() {
    let instruction = Instruction {
        meta_info: InstructionMetaInfo::new(),
        instruction_type: InstructionType::Script(ScriptInstruction::new()),
    };

    let mut context = Context::new();
    let mut env = Env::default();

    let (command_result, output_variable) = run_instruction(
        &mut context.commands,
        &mut context.variables,
        &mut HashMap::new(),
        &vec![],
        instruction,
        0,
        &mut env,
    );

    assert!(output_variable.is_none());
    assert!(test::validate_continue_result(&command_result, None));
}

#[test]
fn run_instruction_script_instruction_unknown_command() {
    let mut script_instruction = ScriptInstruction::new();
    script_instruction.command = Some("test".to_string());

    let instruction = Instruction {
        meta_info: InstructionMetaInfo::new(),
        instruction_type: InstructionType::Script(script_instruction),
    };

    let mut context = Context::new();
    let mut env = Env::default();

    let (command_result, output_variable) = run_instruction(
        &mut context.commands,
        &mut context.variables,
        &mut HashMap::new(),
        &vec![],
        instruction,
        0,
        &mut env,
    );

    assert!(output_variable.is_none());
    assert!(test::validate_crash_result(&command_result));
}

#[test]
fn run_instruction_script_instruction_continue_result_no_output() {
    let mut script_instruction = ScriptInstruction::new();
    script_instruction.command = Some("set".to_string());

    let instruction = Instruction {
        meta_info: InstructionMetaInfo::new(),
        instruction_type: InstructionType::Script(script_instruction),
    };

    let mut context = Context::new();
    let result = context.commands.set(Box::new(SetCommand {}));
    assert!(result.is_ok());
    let mut env = Env::default();

    let (command_result, output_variable) = run_instruction(
        &mut context.commands,
        &mut context.variables,
        &mut HashMap::new(),
        &vec![],
        instruction,
        0,
        &mut env,
    );

    assert!(output_variable.is_none());
    assert!(test::validate_continue_result(&command_result, None));
}

#[test]
fn run_instruction_script_instruction_continue_result_with_output() {
    let mut script_instruction = ScriptInstruction::new();
    script_instruction.command = Some("set".to_string());
    script_instruction.output = Some("out".to_string());
    script_instruction.arguments = Some(vec!["value".to_string()]);

    let instruction = Instruction {
        meta_info: InstructionMetaInfo::new(),
        instruction_type: InstructionType::Script(script_instruction),
    };

    let mut context = Context::new();
    let result = context.commands.set(Box::new(SetCommand {}));
    assert!(result.is_ok());
    let mut env = Env::default();

    let (command_result, output_variable) = run_instruction(
        &mut context.commands,
        &mut context.variables,
        &mut HashMap::new(),
        &vec![],
        instruction,
        0,
        &mut env,
    );

    assert_eq!(output_variable.unwrap(), "out");
    assert!(test::validate_continue_result(
        &command_result,
        Some("value".to_string())
    ));
}

#[test]
fn run_instruction_script_instruction_exit_result_no_output() {
    let mut script_instruction = ScriptInstruction::new();
    script_instruction.command = Some("exit".to_string());

    let instruction = Instruction {
        meta_info: InstructionMetaInfo::new(),
        instruction_type: InstructionType::Script(script_instruction),
    };

    let mut context = Context::new();
    let result = context.commands.set(Box::new(ExitCommand {}));
    assert!(result.is_ok());
    let mut env = Env::default();

    let (command_result, output_variable) = run_instruction(
        &mut context.commands,
        &mut context.variables,
        &mut HashMap::new(),
        &vec![],
        instruction,
        0,
        &mut env,
    );

    assert!(output_variable.is_none());
    assert!(test::validate_exit_result(&command_result, None));
}

#[test]
fn run_instruction_script_instruction_exit_result_with_output() {
    let mut script_instruction = ScriptInstruction::new();
    script_instruction.command = Some("exit".to_string());
    script_instruction.output = Some("out".to_string());
    script_instruction.arguments = Some(vec!["value".to_string()]);

    let instruction = Instruction {
        meta_info: InstructionMetaInfo::new(),
        instruction_type: InstructionType::Script(script_instruction),
    };

    let mut context = Context::new();
    let result = context.commands.set(Box::new(ExitCommand {}));
    assert!(result.is_ok());
    let mut env = Env::default();

    let (command_result, output_variable) = run_instruction(
        &mut context.commands,
        &mut context.variables,
        &mut HashMap::new(),
        &vec![],
        instruction,
        0,
        &mut env,
    );

    assert_eq!(output_variable.unwrap(), "out");
    assert!(test::validate_exit_result(
        &command_result,
        Some("value".to_string())
    ));
}

#[test]
fn run_instruction_script_instruction_goto_label_result_no_output() {
    let mut script_instruction = ScriptInstruction::new();
    script_instruction.command = Some("goto".to_string());

    let instruction = Instruction {
        meta_info: InstructionMetaInfo::new(),
        instruction_type: InstructionType::Script(script_instruction),
    };

    let mut context = Context::new();
    let result = context.commands.set(Box::new(GoToLabelCommand {}));
    assert!(result.is_ok());
    let mut env = Env::default();

    let (command_result, output_variable) = run_instruction(
        &mut context.commands,
        &mut context.variables,
        &mut HashMap::new(),
        &vec![],
        instruction,
        0,
        &mut env,
    );

    assert!(output_variable.is_none());
    assert!(test::validate_goto_label_result(&command_result, None));
}

#[test]
fn run_instruction_script_instruction_goto_label_result_with_output() {
    let mut script_instruction = ScriptInstruction::new();
    script_instruction.command = Some("goto".to_string());
    script_instruction.output = Some("out".to_string());
    script_instruction.arguments = Some(vec!["value".to_string()]);

    let instruction = Instruction {
        meta_info: InstructionMetaInfo::new(),
        instruction_type: InstructionType::Script(script_instruction),
    };

    let mut context = Context::new();
    let result = context.commands.set(Box::new(GoToLabelCommand {}));
    assert!(result.is_ok());
    let mut env = Env::default();

    let (command_result, output_variable) = run_instruction(
        &mut context.commands,
        &mut context.variables,
        &mut HashMap::new(),
        &vec![],
        instruction,
        0,
        &mut env,
    );

    assert_eq!(output_variable.unwrap(), "out");
    assert!(test::validate_goto_label_result(
        &command_result,
        Some("value".to_string())
    ));
}

#[test]
fn run_instruction_script_instruction_goto_line_result_no_output() {
    let mut script_instruction = ScriptInstruction::new();
    script_instruction.command = Some("goto".to_string());

    let instruction = Instruction {
        meta_info: InstructionMetaInfo::new(),
        instruction_type: InstructionType::Script(script_instruction),
    };

    let mut context = Context::new();
    let result = context.commands.set(Box::new(GoToLineCommand {}));
    assert!(result.is_ok());
    let mut env = Env::default();

    let (command_result, output_variable) = run_instruction(
        &mut context.commands,
        &mut context.variables,
        &mut HashMap::new(),
        &vec![],
        instruction,
        0,
        &mut env,
    );

    assert!(output_variable.is_none());
    assert!(test::validate_goto_line_result(&command_result, None));
}

#[test]
fn run_instruction_script_instruction_goto_line_result_with_output() {
    let mut script_instruction = ScriptInstruction::new();
    script_instruction.command = Some("goto".to_string());
    script_instruction.output = Some("out".to_string());
    script_instruction.arguments = Some(vec!["2".to_string()]);

    let instruction = Instruction {
        meta_info: InstructionMetaInfo::new(),
        instruction_type: InstructionType::Script(script_instruction),
    };

    let mut context = Context::new();
    let result = context.commands.set(Box::new(GoToLineCommand {}));
    assert!(result.is_ok());
    let mut env = Env::default();

    let (command_result, output_variable) = run_instruction(
        &mut context.commands,
        &mut context.variables,
        &mut HashMap::new(),
        &vec![],
        instruction,
        0,
        &mut env,
    );

    assert_eq!(output_variable.unwrap(), "out");
    assert!(test::validate_goto_line_result(
        &command_result,
        Some("2".to_string())
    ));
}

#[test]
fn run_instruction_script_instruction_error_result() {
    let mut script_instruction = ScriptInstruction::new();
    script_instruction.command = Some("error".to_string());

    let instruction = Instruction {
        meta_info: InstructionMetaInfo::new(),
        instruction_type: InstructionType::Script(script_instruction),
    };

    let mut context = Context::new();
    let result = context.commands.set(Box::new(ErrorCommand {}));
    assert!(result.is_ok());
    let mut env = Env::default();

    let (command_result, output_variable) = run_instruction(
        &mut context.commands,
        &mut context.variables,
        &mut HashMap::new(),
        &vec![],
        instruction,
        0,
        &mut env,
    );

    assert!(output_variable.is_none());
    assert!(test::validate_error_result(&command_result));
}

#[test]
fn run_instruction_control_characters() {
    let mut script_instruction = ScriptInstruction::new();
    script_instruction.command = Some("set".to_string());
    script_instruction.output = Some("out".to_string());
    script_instruction.arguments = Some(vec!["\\\\".to_string()]);

    let instruction = Instruction {
        meta_info: InstructionMetaInfo::new(),
        instruction_type: InstructionType::Script(script_instruction),
    };

    let mut context = Context::new();
    let result = context.commands.set(Box::new(SetCommand {}));
    assert!(result.is_ok());
    let mut env = Env::default();

    let (command_result, output_variable) = run_instruction(
        &mut context.commands,
        &mut context.variables,
        &mut HashMap::new(),
        &vec![],
        instruction,
        0,
        &mut env,
    );

    assert!(output_variable.is_some());
    assert!(test::validate_continue_result(
        &command_result,
        Some("\\\\".to_string())
    ));
}

#[test]
fn run_on_error_instruction_no_command() {
    let mut commands = Commands::new();
    let mut variables = HashMap::new();
    let mut state = HashMap::new();
    let mut env = Env::default();

    let result = run_on_error_instruction(
        &mut commands,
        &mut variables,
        &mut state,
        &vec![],
        "error".to_string(),
        InstructionMetaInfo::new(),
        &mut env,
    );

    assert!(result.is_ok());
}

#[test]
fn run_on_error_instruction_unknown_command() {
    let mut commands = Commands::new();
    let mut variables = HashMap::new();
    let mut state = HashMap::new();
    let mut env = Env::default();

    commands
        .aliases
        .insert("on_error".to_string(), "badcommand".to_string());

    let result = run_on_error_instruction(
        &mut commands,
        &mut variables,
        &mut state,
        &vec![],
        "error".to_string(),
        InstructionMetaInfo::new(),
        &mut env,
    );

    assert!(result.is_ok());
}

#[test]
fn run_on_error_instruction_exit_response() {
    let mut commands = Commands::new();
    let mut variables = HashMap::new();
    let mut state = HashMap::new();
    let mut env = Env::default();

    let set_result = commands.set(Box::new(ExitCommand {}));
    assert!(set_result.is_ok());
    commands
        .aliases
        .insert("on_error".to_string(), "exit".to_string());

    let result = run_on_error_instruction(
        &mut commands,
        &mut variables,
        &mut state,
        &vec![],
        "error".to_string(),
        InstructionMetaInfo::new(),
        &mut env,
    );

    assert!(result.is_err());
}

#[test]
fn run_on_error_instruction_crash_response() {
    let mut commands = Commands::new();
    let mut variables = HashMap::new();
    let mut state = HashMap::new();
    let mut env = Env::default();

    let set_result = commands.set(Box::new(CrashCommand {}));
    assert!(set_result.is_ok());
    commands
        .aliases
        .insert("on_error".to_string(), "crash".to_string());

    let result = run_on_error_instruction(
        &mut commands,
        &mut variables,
        &mut state,
        &vec![],
        "error".to_string(),
        InstructionMetaInfo::new(),
        &mut env,
    );

    assert!(result.is_err());
}

#[test]
fn run_on_error_instruction_continue_response() {
    let mut commands = Commands::new();
    let mut variables = HashMap::new();
    let mut state = HashMap::new();
    let mut env = Env::default();

    let set_result = commands.set(Box::new(SetCommand {}));
    assert!(set_result.is_ok());
    commands
        .aliases
        .insert("on_error".to_string(), "set".to_string());

    let result = run_on_error_instruction(
        &mut commands,
        &mut variables,
        &mut state,
        &vec![],
        "error".to_string(),
        InstructionMetaInfo::new(),
        &mut env,
    );

    assert!(result.is_ok());
}

#[test]
fn run_on_error_instruction_error_response() {
    let mut commands = Commands::new();
    let mut variables = HashMap::new();
    let mut state = HashMap::new();
    let mut env = Env::default();

    let set_result = commands.set(Box::new(ErrorCommand {}));
    assert!(set_result.is_ok());
    commands
        .aliases
        .insert("on_error".to_string(), "error".to_string());

    let result = run_on_error_instruction(
        &mut commands,
        &mut variables,
        &mut state,
        &vec![],
        "error".to_string(),
        InstructionMetaInfo::new(),
        &mut env,
    );

    assert!(result.is_ok());
}
