use super::*;
use crate::test;
use crate::test::CommandValidation;

#[test]
fn common_functions() {
    test::test_common_command_functions(create(""));
}

#[test]
fn run_no_args() {
    test::run_script_and_error(vec![create("")], "out = ftp_list", "out");
}

#[test]
fn run_valid() {
    test::run_script_and_validate(
        vec![create("")],
        "out = ftp_list --host test.rebex.net --username demo --password password",
        CommandValidation::Contains("out".to_string(), "handle:".to_string()),
    );
}
