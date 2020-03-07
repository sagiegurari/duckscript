use super::*;
use crate::test;

#[test]
fn common_functions() {
    test::test_common_command_functions(create(""));
}

cfg_if::cfg_if! {
    if #[cfg(windows)] {
        #[test]
        fn run_windows() {
            test::run_script_and_error(vec![create("")], "out = os_version", "out");
        }
    } else {
        use crate::test::CommandValidation;

        #[test]
        fn run_valid() {
            test::run_script_and_validate(
                vec![create("")],
                "out = os_version",
                CommandValidation::Ignore,
            );
        }
    }
}
