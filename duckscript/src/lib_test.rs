use super::*;

#[test]
fn version_test() {
    let version = version();

    assert!(!version.is_empty());
}
