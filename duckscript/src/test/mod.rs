use crate::types::command::{Command, CommandResult};
use crate::types::instruction::InstructionMetaInfo;
use crate::types::instruction::{
    Instruction, InstructionType, PreProcessInstruction, ScriptInstruction,
};
use crate::types::runtime::Context;
use std::cell::RefCell;
use std::rc::Rc;

pub struct SetCommand {}

impl Command for SetCommand {
    fn name(&self) -> String {
        "set".to_string()
    }

    fn aliases(&self) -> Vec<String> {
        vec![]
    }

    fn run(
        &self,
        _context: Rc<RefCell<&Context>>,
        arguments: Vec<String>,
        _meta_info: &InstructionMetaInfo,
    ) -> CommandResult {
        let output = if arguments.is_empty() {
            None
        } else {
            Some(arguments[0].clone())
        };

        CommandResult::Continue(output)
    }
}

pub struct ExitCommand {}

impl Command for ExitCommand {
    fn name(&self) -> String {
        "exit".to_string()
    }

    fn aliases(&self) -> Vec<String> {
        vec![]
    }

    fn run(
        &self,
        _context: Rc<RefCell<&Context>>,
        arguments: Vec<String>,
        _meta_info: &InstructionMetaInfo,
    ) -> CommandResult {
        let output = if arguments.is_empty() {
            None
        } else {
            Some(arguments[0].clone())
        };

        CommandResult::Exit(output)
    }
}

pub struct ErrorCommand {}

impl Command for ErrorCommand {
    fn name(&self) -> String {
        "error".to_string()
    }

    fn aliases(&self) -> Vec<String> {
        vec![]
    }

    fn run(
        &self,
        _context: Rc<RefCell<&Context>>,
        _arguments: Vec<String>,
        meta_info: &InstructionMetaInfo,
    ) -> CommandResult {
        CommandResult::Error("test".to_string(), meta_info.clone())
    }
}

pub struct TestCommand1 {}

impl Command for TestCommand1 {
    fn name(&self) -> String {
        "test1".to_string()
    }

    fn aliases(&self) -> Vec<String> {
        vec!["test11".to_string(), "test12".to_string()]
    }

    fn run(
        &self,
        _context: Rc<RefCell<&Context>>,
        _arguments: Vec<String>,
        _meta_info: &InstructionMetaInfo,
    ) -> CommandResult {
        CommandResult::Continue(None)
    }
}

pub struct TestCommand2 {}

impl Command for TestCommand2 {
    fn name(&self) -> String {
        "test2".to_string()
    }

    fn aliases(&self) -> Vec<String> {
        vec!["test21".to_string(), "test22".to_string()]
    }

    fn run(
        &self,
        _context: Rc<RefCell<&Context>>,
        _arguments: Vec<String>,
        _meta_info: &InstructionMetaInfo,
    ) -> CommandResult {
        CommandResult::Continue(None)
    }
}

pub struct TestCommand3 {}

impl Command for TestCommand3 {
    fn name(&self) -> String {
        "test1".to_string()
    }

    fn aliases(&self) -> Vec<String> {
        vec!["test3".to_string()]
    }

    fn run(
        &self,
        _context: Rc<RefCell<&Context>>,
        _arguments: Vec<String>,
        _meta_info: &InstructionMetaInfo,
    ) -> CommandResult {
        CommandResult::Continue(None)
    }
}

pub struct TestCommand4 {}

impl Command for TestCommand4 {
    fn name(&self) -> String {
        "test4".to_string()
    }

    fn aliases(&self) -> Vec<String> {
        vec!["test11".to_string()]
    }

    fn run(
        &self,
        _context: Rc<RefCell<&Context>>,
        _arguments: Vec<String>,
        _meta_info: &InstructionMetaInfo,
    ) -> CommandResult {
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
