use super::*;
use duckscript::runner;
use duckscript::types::runtime::Context;

#[test]
fn version_test() {
    let version = version();

    assert!(!version.is_empty());
}

#[test]
fn load_valid() {
    let mut context = Context::new();
    let result = load(&mut context.commands);

    match result {
        Ok(_) => (),
        Err(e) => panic!("{:#?}", e),
    };
    //  assert!(result.is_ok());
    assert!(!context.commands.get_all_command_names().is_empty());

    let result = runner::run_script("test_directory ../test", context);

    assert!(result.is_ok());
}
