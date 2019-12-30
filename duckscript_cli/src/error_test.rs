use super::*;

use duckscript::types::error;

#[test]
fn display_script() {
    let error = CliError {
        info: ErrorInfo::Script(ScriptError {
            info: error::ErrorInfo::Initialization("test".to_string()),
        }),
    };
    println!("{}", error);
}

#[test]
fn display_cli() {
    let error = CliError {
        info: ErrorInfo::Cli("test"),
    };
    println!("{}", error);
}
