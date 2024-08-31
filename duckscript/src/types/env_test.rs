use super::*;

fn validate_env(env: &mut Env) {
    writeln!(env.out, "test").unwrap();
}

#[test]
fn env_default() {
    let mut env = Env::default();

    validate_env(&mut env);
}

#[test]
fn env_new_none() {
    let mut env = Env::new(None, None);

    validate_env(&mut env);
}

#[test]
fn env_new_with_values() {
    let mut env = Env::new(Some(Box::new(stdout())), Some(Box::new(stdout())));

    validate_env(&mut env);
}
