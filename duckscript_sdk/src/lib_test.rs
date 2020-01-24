use super::*;
use duckscript::types::command::Commands;

#[test]
fn version_test() {
    let version = version();

    assert!(!version.is_empty());
}

#[test]
fn load_valid() {
    let mut commands = Commands::new();
    let result = load(&mut commands);

    assert!(result.is_ok());
    assert!(!commands.get_all_command_names().is_empty());
}
