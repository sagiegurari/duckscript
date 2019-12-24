use crate::utils::io;
use duckscript::types::command::{Command, CommandResult};
use duckscript::types::instruction::InstructionMetaInfo;
use duckscript::types::runtime::Context;
use std::cell::RefCell;
use std::rc::Rc;

#[cfg(test)]
#[path = "./mod_test.rs"]
mod mod_test;

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
            let cntxt = context.borrow();
            let names = cntxt.commands.get_all_command_names();
            let mut buffer = String::new();

            // create ToC
            buffer.push_str("# Table of Contents\n");
            for name in &names {
                buffer.push_str(&format!("* [{}](#{})\n", name, name.replace(":", "_")));
            }

            // create doc per command
            buffer.push_str("\n");
            for name in &names {
                let command = match cntxt.commands.get(name) {
                    Some(command) => command,
                    None => {
                        return CommandResult::Error(
                            format!("Command: {} not found", name),
                            meta_info.clone(),
                        );
                    }
                };

                let aliases = command.aliases();
                let aliases_docs = if !aliases.is_empty() {
                    let mut aliases_docs_buffer = String::from("\n\n#### Aliases:\n");

                    let mut first = true;
                    for alias in aliases {
                        if first {
                            first = false;
                        } else {
                            aliases_docs_buffer.push_str(", ");
                        }

                        aliases_docs_buffer.push_str(&alias);
                    }

                    aliases_docs_buffer
                } else {
                    "".to_string()
                };

                let help = command.help();
                buffer.push_str(&format!(
                    "\n<a name=\"{}\"></a>\n## {}\n{}{}\n",
                    name.replace(":", "_"),
                    name,
                    help,
                    aliases_docs
                ));
            }

            // footer
            buffer.push_str(include_str!("footer.md"));

            let file = arguments[0].clone();

            match io::write_text_file(&file, &buffer, &meta_info) {
                Ok(_) => CommandResult::Continue(Some(file)),
                Err(error) => CommandResult::Error(error.to_string(), meta_info.clone()),
            }
        }
    }
}

pub(crate) fn create(package: &str) -> Box<dyn Command> {
    Box::new(CommandImpl {
        package: package.to_string(),
    })
}
