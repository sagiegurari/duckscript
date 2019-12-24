use duckscript::types::error::{ErrorInfo, ScriptError};
use duckscript::types::instruction::InstructionMetaInfo;
use std::fs::File;
use std::io::Write;
use std::path::Path;

pub(crate) fn write_text_file(
    file: &str,
    text: &str,
    meta_info: &InstructionMetaInfo,
) -> Result<(), ScriptError> {
    let file_path = Path::new(file);

    match File::create(&file_path) {
        Ok(mut fd) => match fd.write_all(text.as_bytes()) {
            Err(_) => Err(ScriptError {
                info: ErrorInfo::Runtime(
                    format!("Error writing to file: {}", file).to_string(),
                    meta_info.clone(),
                ),
            }),
            Ok(_) => match fd.sync_all() {
                _ => Ok(()),
            },
        },
        Err(_) => Err(ScriptError {
            info: ErrorInfo::Runtime(
                format!("Error opening file: {} for writing.", file).to_string(),
                meta_info.clone(),
            ),
        }),
    }
}
