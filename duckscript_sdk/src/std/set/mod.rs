use duckscript::types::command::{Command, CommandResult};
use duckscript::types::instruction::InstructionMetaInfo;
use duckscript::types::runtime::Context;
use std::cell::RefCell;
use std::rc::Rc;

struct CommandImpl {}

impl Command for CommandImpl {
    fn name(&self) -> String {
        "std::Set".to_string()
    }

    fn aliases(&self) -> Vec<String> {
        vec!["set".to_string()]
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

pub(crate) fn create() -> Box<dyn Command> {
    Box::new(CommandImpl {})
}
