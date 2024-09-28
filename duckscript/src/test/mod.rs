use crate::types::command::{Command, CommandResult, Commands, GoToValue};
use crate::types::env::Env;
use crate::types::instruction::{
    Instruction, InstructionType, PreProcessInstruction, ScriptInstruction,
};
use crate::types::runtime::StateValue;
use std::collections::HashMap;

#[derive(Clone)]
pub(crate) struct SetCommand {}

impl Command for SetCommand {
    fn name(&self) -> String {
        "set".to_string()
    }

    fn clone_and_box(&self) -> Box<dyn Command> {
        Box::new((*self).clone())
    }

    fn run(&self, arguments: Vec<String>) -> CommandResult {
        let output = if arguments.is_empty() {
            None
        } else {
            Some(arguments[0].clone())
        };

        CommandResult::Continue(output)
    }
}

#[derive(Clone)]
pub(crate) struct ExitCommand {}

impl Command for ExitCommand {
    fn name(&self) -> String {
        "exit".to_string()
    }

    fn clone_and_box(&self) -> Box<dyn Command> {
        Box::new((*self).clone())
    }

    fn run(&self, arguments: Vec<String>) -> CommandResult {
        let output = if arguments.is_empty() {
            None
        } else {
            Some(arguments[0].clone())
        };

        CommandResult::Exit(output)
    }
}

#[derive(Clone)]
pub(crate) struct OnErrorCommand {}

impl Command for OnErrorCommand {
    fn name(&self) -> String {
        "on_error".to_string()
    }

    fn clone_and_box(&self) -> Box<dyn Command> {
        Box::new((*self).clone())
    }

    fn requires_context(&self) -> bool {
        true
    }

    fn run_with_context(
        &self,
        arguments: Vec<String>,
        _state: &mut HashMap<String, StateValue>,
        variables: &mut HashMap<String, String>,
        _output_variable: Option<String>,
        _instructions: &Vec<Instruction>,
        _commands: &mut Commands,
        _line: usize,
        env: &mut Env,
    ) -> CommandResult {
        let mut index = 0;
        for argument in arguments {
            index = index + 1;
            variables.insert(index.to_string(), argument.clone());
        }

        writeln!(env.out, "{}", "test").unwrap();

        CommandResult::Continue(None)
    }
}

#[derive(Clone)]
pub(crate) struct ErrorCommand {}

impl Command for ErrorCommand {
    fn name(&self) -> String {
        "error".to_string()
    }

    fn clone_and_box(&self) -> Box<dyn Command> {
        Box::new((*self).clone())
    }

    fn run(&self, _arguments: Vec<String>) -> CommandResult {
        CommandResult::Error("test".to_string())
    }
}

#[derive(Clone)]
pub(crate) struct CrashCommand {}

impl Command for CrashCommand {
    fn name(&self) -> String {
        "crash".to_string()
    }

    fn clone_and_box(&self) -> Box<dyn Command> {
        Box::new((*self).clone())
    }

    fn run(&self, _arguments: Vec<String>) -> CommandResult {
        CommandResult::Crash("test".to_string())
    }
}

#[derive(Clone)]
pub(crate) struct GoToLabelCommand {}

impl Command for GoToLabelCommand {
    fn name(&self) -> String {
        "goto".to_string()
    }

    fn clone_and_box(&self) -> Box<dyn Command> {
        Box::new((*self).clone())
    }

    fn run(&self, arguments: Vec<String>) -> CommandResult {
        let (output, label) = if arguments.is_empty() {
            (None, "target".to_string())
        } else {
            (Some(arguments[0].clone()), arguments[0].clone())
        };

        CommandResult::GoTo(output, GoToValue::Label(label))
    }
}

#[derive(Clone)]
pub(crate) struct GoToLineCommand {}

impl Command for GoToLineCommand {
    fn name(&self) -> String {
        "goto".to_string()
    }

    fn clone_and_box(&self) -> Box<dyn Command> {
        Box::new((*self).clone())
    }

    fn run(&self, arguments: Vec<String>) -> CommandResult {
        let (output, line) = if arguments.is_empty() {
            (None, 900)
        } else {
            (
                Some(arguments[0].clone()),
                arguments[0].clone().parse().unwrap(),
            )
        };

        CommandResult::GoTo(output, GoToValue::Line(line))
    }
}

#[derive(Clone)]
pub(crate) struct TestCommand1 {}

impl Command for TestCommand1 {
    fn name(&self) -> String {
        "test1".to_string()
    }

    fn aliases(&self) -> Vec<String> {
        vec!["test11".to_string(), "test12".to_string()]
    }

    fn clone_and_box(&self) -> Box<dyn Command> {
        Box::new((*self).clone())
    }

    fn run(&self, _arguments: Vec<String>) -> CommandResult {
        CommandResult::Continue(None)
    }
}

#[derive(Clone)]
pub(crate) struct TestCommand2 {}

impl Command for TestCommand2 {
    fn name(&self) -> String {
        "test2".to_string()
    }

    fn aliases(&self) -> Vec<String> {
        vec!["test21".to_string(), "test22".to_string()]
    }

    fn clone_and_box(&self) -> Box<dyn Command> {
        Box::new((*self).clone())
    }

    fn run(&self, _arguments: Vec<String>) -> CommandResult {
        CommandResult::Continue(None)
    }
}

#[derive(Clone)]
pub(crate) struct TestCommand3 {}

impl Command for TestCommand3 {
    fn name(&self) -> String {
        "test1".to_string()
    }

    fn aliases(&self) -> Vec<String> {
        vec!["test3".to_string()]
    }

    fn clone_and_box(&self) -> Box<dyn Command> {
        Box::new((*self).clone())
    }

    fn run(&self, _arguments: Vec<String>) -> CommandResult {
        CommandResult::Continue(None)
    }
}

#[derive(Clone)]
pub(crate) struct TestCommand4 {}

impl Command for TestCommand4 {
    fn name(&self) -> String {
        "test4".to_string()
    }

    fn aliases(&self) -> Vec<String> {
        vec!["test11".to_string()]
    }

    fn clone_and_box(&self) -> Box<dyn Command> {
        Box::new((*self).clone())
    }

    fn run(&self, _arguments: Vec<String>) -> CommandResult {
        CommandResult::Continue(None)
    }
}

pub(crate) fn get_pre_process_instruction(instruction: &Instruction) -> PreProcessInstruction {
    match &instruction.instruction_type {
        InstructionType::PreProcess(value) => value.clone(),
        _ => panic!("Wrong instruction type."),
    }
}

pub(crate) fn get_script_instruction(instruction: &Instruction) -> ScriptInstruction {
    match &instruction.instruction_type {
        InstructionType::Script(value) => value.clone(),
        _ => panic!("Wrong instruction type."),
    }
}

pub(crate) fn assert_empty_instruction(instruction: &Instruction) {
    match &instruction.instruction_type {
        InstructionType::Empty => (),
        _ => panic!("Wrong instruction type."),
    };
}

pub(crate) fn validate_continue_result(result: &CommandResult, value: Option<String>) -> bool {
    match result {
        CommandResult::Continue(output) => {
            assert_eq!(output, &value);
            true
        }
        _ => false,
    }
}

pub(crate) fn validate_exit_result(result: &CommandResult, value: Option<String>) -> bool {
    match result {
        CommandResult::Exit(output) => {
            assert_eq!(output, &value);
            true
        }
        _ => false,
    }
}

pub(crate) fn validate_goto_label_result(result: &CommandResult, value: Option<String>) -> bool {
    match result {
        CommandResult::GoTo(output, label) => {
            assert_eq!(output, &value);
            if value.is_some() {
                match label {
                    GoToValue::Label(label_text) => {
                        assert_eq!(output, &Some(label_text.to_string()));
                        true
                    }
                    _ => false,
                }
            } else {
                match label {
                    GoToValue::Label(label_text) => {
                        assert_eq!("target", label_text);
                        true
                    }
                    _ => false,
                }
            }
        }
        _ => false,
    }
}

pub(crate) fn validate_goto_line_result(result: &CommandResult, value: Option<String>) -> bool {
    match result {
        CommandResult::GoTo(output, _) => {
            assert_eq!(output, &value);
            true
        }
        _ => false,
    }
}

pub(crate) fn validate_error_result(result: &CommandResult) -> bool {
    match result {
        CommandResult::Error(_) => true,
        _ => false,
    }
}

pub(crate) fn validate_crash_result(result: &CommandResult) -> bool {
    match result {
        CommandResult::Crash(_) => true,
        _ => false,
    }
}
