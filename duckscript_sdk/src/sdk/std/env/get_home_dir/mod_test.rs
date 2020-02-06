use super::*;
use crate::test;
use crate::test::CommandValidation;

#[test]
fn common_functions() {
    test::test_common_command_functions(create(""));
}

#[test]
fn run_valid() {
    let directory = home::home_dir().unwrap().to_string_lossy().into_owned();
    test::run_script_and_validate(
        vec![create("")],
        "out = get_home_dir",
        CommandValidation::Match("out".to_string(), directory),
    );
}
