use super::*;
use crate::test;
use crate::test::CommandValidation;
use fsio::file::ensure_exists;

#[test]
fn common_functions() {
    test::test_common_command_functions(create(""));
}

#[test]
fn run_no_path_no_mode_provided() {
    test::run_script_and_error(vec![create("")], "out = chmod", "out");
}

#[test]
fn run_no_path_provided() {
    test::run_script_and_error(vec![create("")], "out = chmod 755", "out");
}

#[cfg(any(target_os = "redox", unix, target_os = "vxworks"))]
#[test]
fn run_single_file_unix() {
    let result = ensure_exists("./target/_duckscript/chmod/file.txt");
    assert!(result.is_ok());

    test::run_script_and_validate(
        vec![create("")],
        "out = chmod 777 ./target/_duckscript/chmod/file.txt",
        CommandValidation::Match("out".to_string(), "511".to_string()),
    );
}
