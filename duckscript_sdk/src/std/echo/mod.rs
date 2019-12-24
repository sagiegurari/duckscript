use duckscript::types::command::{Command, CommandResult};
use duckscript::types::instruction::InstructionMetaInfo;
use duckscript::types::runtime::Context;
use std::cell::RefCell;
use std::rc::Rc;

struct CommandImpl {}

impl Command for CommandImpl {
    fn name(&self) -> String {
        "std::Echo".to_string()
    }

    fn aliases(&self) -> Vec<String> {
        vec!["echo".to_string()]
    }

    fn help(&self) -> String {
        format!("No documentation found for command: {}", self.name())
    }

    fn run(
        &self,
        _context: Rc<RefCell<&Context>>,
        arguments: Vec<String>,
        _meta_info: &InstructionMetaInfo,
    ) -> CommandResult {
        for argument in &arguments {
            print!("{} ", argument);
        }

        println!("");

        CommandResult::Continue(None)
    }
}

pub(crate) fn create() -> Box<dyn Command> {
    Box::new(CommandImpl {})
}
