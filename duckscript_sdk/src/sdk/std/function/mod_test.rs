use super::*;
use crate::test;

#[test]
fn common_functions() {
    let commands = create("");
    for command in commands {
        test::test_common_command_functions(command);
    }
}
