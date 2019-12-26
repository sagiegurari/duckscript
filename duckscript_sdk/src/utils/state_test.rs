use super::*;

#[test]
fn get_core_sub_state_for_command_valid() {
    let mut state = HashMap::new();
    let value = get_core_sub_state_for_command(&mut state, "test".to_string());

    assert!(value.is_empty());
    assert!(state.contains_key("duckscript::test"));
}

#[test]
fn ensure_sub_state_empty() {
    let mut state = HashMap::new();
    ensure_sub_state("test", &mut state);

    assert!(state.get("test").is_some());
}

#[test]
fn ensure_sub_state_diff_type() {
    let mut state = HashMap::new();
    state.insert("test".to_string(), StateValue::Number(1));
    ensure_sub_state("test", &mut state);

    match state.get("test").unwrap() {
        StateValue::SubState(_) => (),
        _ => panic!("invalid type"),
    };
}

#[test]
fn ensure_sub_state_same_type() {
    let mut sub_state = HashMap::new();
    sub_state.insert("sub".to_string(), StateValue::Number(1));
    let mut state = HashMap::new();
    state.insert("test".to_string(), StateValue::SubState(sub_state));
    ensure_sub_state("test", &mut state);

    match state.get("test").unwrap() {
        StateValue::SubState(value) => assert!(!value.is_empty()),
        _ => panic!("invalid type"),
    };
}

#[test]
fn get_sub_state_valid() {
    let mut state = HashMap::new();
    let value = get_sub_state("test".to_string(), &mut state);

    assert!(value.is_empty());
    assert!(state.contains_key("test"));
}

#[test]
fn ensure_list_empty() {
    let mut state = HashMap::new();
    ensure_list("test", &mut state);

    assert!(state.get("test").is_some());
}

#[test]
fn ensure_list_diff_type() {
    let mut state = HashMap::new();
    state.insert("test".to_string(), StateValue::Number(1));
    ensure_list("test", &mut state);

    match state.get("test").unwrap() {
        StateValue::List(_) => (),
        _ => panic!("invalid type"),
    };
}

#[test]
fn ensure_list_same_type() {
    let mut list = vec![];
    list.push(StateValue::Number(1));
    let mut state = HashMap::new();
    state.insert("test".to_string(), StateValue::List(list));
    ensure_list("test", &mut state);

    match state.get("test").unwrap() {
        StateValue::List(value) => assert_eq!(value.len(), 1),
        _ => panic!("invalid type"),
    };
}

#[test]
fn get_list_valid() {
    let mut state = HashMap::new();
    let value = get_list("test".to_string(), &mut state);

    assert!(value.is_empty());
    assert!(state.contains_key("test"));
}
