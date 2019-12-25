use super::*;

#[test]
fn context_new() {
    let mut context = Context::new();

    assert!(context.variables.is_empty());
    assert!(context.state.is_empty());
    assert!(context.commands.get_for_use("test").is_none());
}

#[test]
fn runtime_new() {
    let mut context = Context::new();
    context.variables.insert("a".to_string(), "b".to_string());

    let runtime = Runtime::new(context);

    assert!(runtime.instructions.is_none());
    assert!(runtime.label_to_line.is_empty());
    assert!(runtime.context.variables.contains_key("a"));
}
