use super::*;

use crate::test;
use crate::test::{ErrorCommand, ExitCommand, GoToLabelCommand, GoToLineCommand, SetCommand};
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

    let runtime = create_runtime(instructions, Context::new());

    let context_result = run_instructions(runtime, 0);

    assert!(context_result.is_ok());
    assert!(context_result.unwrap().variables.is_empty());
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

    let runtime = create_runtime(instructions, context);

    let context_result = run_instructions(runtime, 0);

    assert!(context_result.is_ok());

    context = context_result.unwrap();
    assert_eq!(context.variables.get("out1"), Some(&"value1".to_string()));
    assert_eq!(context.variables.get("out2"), Some(&"value2".to_string()));
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

    let runtime = create_runtime(instructions, context);

    let context_result = run_instructions(runtime, 0);

    assert!(context_result.is_ok());

    context = context_result.unwrap();
    assert_eq!(context.variables.get("out2"), Some(&"value2".to_string()));
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

    let runtime = create_runtime(instructions, context);

    let context_result = run_instructions(runtime, 0);

    assert!(context_result.is_ok());

    context = context_result.unwrap();
    assert_eq!(context.variables.get("out1"), Some(&"my_label".to_string()));
    assert_eq!(context.variables.get("out2"), Some(&"value2".to_string()));
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

    let runtime = create_runtime(instructions, context);

    let context_result = run_instructions(runtime, 0);

    assert!(context_result.is_ok());

    context = context_result.unwrap();
    assert_eq!(context.variables.get("out2"), Some(&"value2".to_string()));
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

    let runtime = create_runtime(instructions, context);

    let context_result = run_instructions(runtime, 0);

    assert!(context_result.is_ok());

    context = context_result.unwrap();
    assert_eq!(context.variables.get("out1"), Some(&"2".to_string()));
    assert_eq!(context.variables.get("out2"), Some(&"value2".to_string()));
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

    let arguments = bind_command_arguments(&context.variables, &script_instruction);

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

    let (command_result, output_variable) = run_instruction(
        &mut context.commands,
        &mut context.variables,
        &mut HashMap::new(),
        &vec![],
        instruction,
        0,
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

    let (command_result, output_variable) = run_instruction(
        &mut context.commands,
        &mut context.variables,
        &mut HashMap::new(),
        &vec![],
        instruction,
        0,
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

    let (command_result, output_variable) = run_instruction(
        &mut context.commands,
        &mut context.variables,
        &mut HashMap::new(),
        &vec![],
        instruction,
        0,
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

    let (command_result, output_variable) = run_instruction(
        &mut context.commands,
        &mut context.variables,
        &mut HashMap::new(),
        &vec![],
        instruction,
        0,
    );

    assert!(output_variable.is_none());
    assert!(test::validate_error_result(&command_result));
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

    let (command_result, output_variable) = run_instruction(
        &mut context.commands,
        &mut context.variables,
        &mut HashMap::new(),
        &vec![],
        instruction,
        0,
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

    let (command_result, output_variable) = run_instruction(
        &mut context.commands,
        &mut context.variables,
        &mut HashMap::new(),
        &vec![],
        instruction,
        0,
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

    let (command_result, output_variable) = run_instruction(
        &mut context.commands,
        &mut context.variables,
        &mut HashMap::new(),
        &vec![],
        instruction,
        0,
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

    let (command_result, output_variable) = run_instruction(
        &mut context.commands,
        &mut context.variables,
        &mut HashMap::new(),
        &vec![],
        instruction,
        0,
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

    let (command_result, output_variable) = run_instruction(
        &mut context.commands,
        &mut context.variables,
        &mut HashMap::new(),
        &vec![],
        instruction,
        0,
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

    let (command_result, output_variable) = run_instruction(
        &mut context.commands,
        &mut context.variables,
        &mut HashMap::new(),
        &vec![],
        instruction,
        0,
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

    let (command_result, output_variable) = run_instruction(
        &mut context.commands,
        &mut context.variables,
        &mut HashMap::new(),
        &vec![],
        instruction,
        0,
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

    let (command_result, output_variable) = run_instruction(
        &mut context.commands,
        &mut context.variables,
        &mut HashMap::new(),
        &vec![],
        instruction,
        0,
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

    let (command_result, output_variable) = run_instruction(
        &mut context.commands,
        &mut context.variables,
        &mut HashMap::new(),
        &vec![],
        instruction,
        0,
    );

    assert!(output_variable.is_none());
    assert!(test::validate_error_result(&command_result));
}
