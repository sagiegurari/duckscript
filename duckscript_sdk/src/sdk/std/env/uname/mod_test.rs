use super::*;
use crate::test;

#[test]
fn common_functions() {
    test::test_common_command_functions(create("").unwrap());
}
