use std::fs::OpenOptions;
use std::path::Path;

use zip::ZipArchive;

use duckscript::types::command::{Command, CommandArgs, CommandResult};

use crate::utils::pckg;

#[derive(Clone)]
pub(crate) struct CommandImpl {
    package: String,
}

impl Command for CommandImpl {
    fn name(&self) -> String {
        pckg::concat(&self.package, "Unzip")
    }

    fn aliases(&self) -> Vec<String> {
        vec!["unzip".to_string()]
    }

    fn help(&self) -> String {
        include_str!("help.md").to_string()
    }

    fn clone_and_box(&self) -> Box<dyn Command> {
        Box::new((*self).clone())
    }

    fn run(&self, arguments: CommandArgs) -> CommandResult {
        if arguments.args.len() < 2 {
            return CommandResult::Error(
                "Paths to the ZIP file and/or target directory are not provided.".to_string(),
            );
        }

        let zipfile = Path::new(&arguments.args[0]);
        let target_dir = Path::new(&arguments.args[1]);

        match std::fs::create_dir_all(target_dir) {
            Ok(_) => (),
            Err(err) => {
                return CommandResult::Error(format!("Couldn't create target directory: {}", err))
            }
        };

        let zip_file = match OpenOptions::new().read(true).open(zipfile) {
            Ok(file) => file,
            Err(err) => return CommandResult::Error(format!("Couldn't open ZIP file: {}", err)),
        };

        let mut zip = match ZipArchive::new(zip_file) {
            Ok(archive) => archive,
            Err(err) => return CommandResult::Error(format!("Couldn't open ZIP file: {}", err)),
        };

        for file in zip.file_names() {
            let file_path = target_dir.join(file);
            if file_path.exists() && file_path.is_file() {
                return CommandResult::Error(format!(
                    "File already exists: {}",
                    file_path.to_str().unwrap()
                ));
            }
        }

        match zip.extract(target_dir) {
            Ok(_) => (),
            Err(err) => return CommandResult::Error(format!("Couldn't unpack ZIP file: {}", err)),
        };

        CommandResult::Continue(Some("true".to_string()))
    }
}

pub(crate) fn create(package: &str) -> Box<dyn Command> {
    Box::new(CommandImpl {
        package: package.to_string(),
    })
}
