use super::*;

#[test]
fn expand_by_wrapper_found() {
    let mut variables = HashMap::new();
    variables.insert("FOUND1".to_string(), "test1".to_string());
    variables.insert("FOUND2".to_string(), "test2".to_string());
    variables.insert("FOUND3".to_string(), "test3".to_string());
    variables.insert("FOUND4".to_string(), "test4".to_string());

    let output = expand_by_wrapper(
        r#"
value1:${FOUND1}
value2:${FOUND2}
value3:${FOUND3}
value4:${FOUND4}

value1:${FOUND1}
value2:${FOUND2}
value3:${FOUND3}
value4:${FOUND4}
    "#,
        "${",
        '}',
        &mut variables,
    );

    assert_eq!(
        r#"
value1:test1
value2:test2
value3:test3
value4:test4

value1:test1
value2:test2
value3:test3
value4:test4
    "#,
        output
    );
}

#[test]
fn expand_by_wrapper_partial_found() {
    let mut variables = HashMap::new();
    variables.insert("PARTIAL_FOUND1".to_string(), "test1".to_string());
    variables.insert("PARTIAL_FOUND2".to_string(), "test2".to_string());
    variables.insert("PARTIAL_FOUND3".to_string(), "test3".to_string());

    let output = expand_by_wrapper(
        r#"
value1:${PARTIAL_FOUND1}
value2:${PARTIAL_FOUND2}
value3:${PARTIAL_FOUND3}
value4:${PARTIAL_FOUND4}

value1:${PARTIAL_FOUND1}
value2:${PARTIAL_FOUND2}
value3:${PARTIAL_FOUND3}
value4:${PARTIAL_FOUND4}
    "#,
        "${",
        '}',
        &mut variables,
    );

    assert_eq!(
        r#"
value1:test1
value2:test2
value3:test3
value4:

value1:test1
value2:test2
value3:test3
value4:
    "#,
        output
    );
}

#[test]
fn expand_by_wrapper_no_suffix() {
    let mut variables = HashMap::new();
    variables.insert("NO_SUFFIX1".to_string(), "test1".to_string());
    variables.insert("NO_SUFFIX2".to_string(), "test2".to_string());

    let output = expand_by_wrapper(
        r#"
value1:${NO_SUFFIX1}
value2:${NO_SUFFIX2}
value3:${NO_SUFFIX3

value1:${NO_SUFFIX1}
value2:${NO_SUFFIX2}
value3:${NO_SUFFIX3
    "#,
        "${",
        '}',
        &mut variables,
    );

    assert_eq!(
        r#"
value1:test1
value2:test2
value3:${NO_SUFFIX3

value1:test1
value2:test2
value3:${NO_SUFFIX3
    "#,
        output
    );
}

#[test]
fn expand_by_wrapper_no_suffix_single() {
    let mut variables = HashMap::new();
    let output = expand_by_wrapper("${NO_SUFFIX_SINGLE", "${", '}', &mut variables);

    assert_eq!("${NO_SUFFIX_SINGLE", output);
}

#[test]
fn expand_by_wrapper_with_escape() {
    let mut variables = HashMap::new();
    variables.insert("FOUND1".to_string(), "test1".to_string());
    variables.insert("FOUND2".to_string(), "test2".to_string());
    variables.insert("FOUND3".to_string(), "test3".to_string());
    variables.insert("FOUND4".to_string(), "test4".to_string());

    let output = expand_by_wrapper(
        r#"
value1:\${FOUND1}
value2:\${FOUND2}
value3:\${FOUND3}
value4:\${FOUND4}

value1:${FOUND1}
value2:${FOUND2}
value3:${FOUND3}
value4:${FOUND4}
    "#,
        "${",
        '}',
        &mut variables,
    );

    assert_eq!(
        r#"
value1:${FOUND1}
value2:${FOUND2}
value3:${FOUND3}
value4:${FOUND4}

value1:test1
value2:test2
value3:test3
value4:test4
    "#,
        output
    );
}
