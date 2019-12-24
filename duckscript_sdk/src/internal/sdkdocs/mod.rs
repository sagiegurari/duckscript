use duckscript::types::command::{Command, CommandResult};
use duckscript::types::instruction::InstructionMetaInfo;
use duckscript::types::runtime::Context;
use std::cell::RefCell;
use std::rc::Rc;

struct CommandImpl {
    package: String,
}

impl Command for CommandImpl {
    fn name(&self) -> String {
        format!("{}::sdkdocs", &self.package).to_string()
    }

    fn aliases(&self) -> Vec<String> {
        vec![]
    }

    fn help(&self) -> String {
        include_str!("help.md").to_string()
    }

    fn run(
        &self,
        context: Rc<RefCell<&Context>>,
        arguments: Vec<String>,
        meta_info: &InstructionMetaInfo,
    ) -> CommandResult {
        if arguments.is_empty() {
            CommandResult::Error(
                "Documentation output directory not provided.".to_string(),
                meta_info.clone(),
            )
        } else {
            let directory = arguments[0].clone();

            CommandResult::Continue(Some(directory))
        }
    }
}

pub(crate) fn create(package: &str) -> Box<dyn Command> {
    Box::new(CommandImpl {
        package: package.to_string(),
    })
}
