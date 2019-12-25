use super::*;

#[test]
fn concat_both_provided() {
    let output = concat("1::2", "3");
    assert_eq!(output, "1::2::3");
}

#[test]
fn concat_parent_provided() {
    let output = concat("1::2", "");
    assert_eq!(output, "1::2");
}

#[test]
fn concat_current_provided() {
    let output = concat("", "3::4");
    assert_eq!(output, "3::4");
}
