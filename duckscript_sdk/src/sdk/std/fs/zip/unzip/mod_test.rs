use super::*;
use crate::test;
use crate::test::CommandValidation;

#[test]
fn common_functions() {
    test::test_common_command_functions(create(""));
}

#[test]
fn run_no_args() {
    test::run_script_and_error(vec![create("")], "out = unzip", "out");
}

#[test]
fn run_only_zip_file() {
    test::run_script_and_error(vec![create("")], "out = unzip ./myfile.zip", "out");
}

#[test]
fn run_only_zip_file_not_exists() {
    test::run_script_and_error(vec![create("")], "out = unzip ./myfile.zip ./out", "out");
}
