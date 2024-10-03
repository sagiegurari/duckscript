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

    assert!(result.is_ok());

    assert!(!context.commands.get_all_command_names().is_empty());
}

#[test]
#[ignore]
fn test_scripts() {
    let mut context = Context::new();
    let result = load(&mut context.commands);

    assert!(result.is_ok());

    assert!(!context.commands.get_all_command_names().is_empty());

    let result = runner::run_script(
        r#"
    set_env DUCKSCRIPT_TEST_RUST true
    test_directory ../test
    unset_env DUCKSCRIPT_TEST_RUST
    "#,
        context,
        None,
    );

    assert!(result.is_ok());
}
