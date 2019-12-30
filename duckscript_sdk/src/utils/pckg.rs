#[cfg(test)]
#[path = "./pckg_test.rs"]
mod pckg_test;

pub(crate) fn concat(parent: &str, current: &str) -> String {
    let mut package = String::from(parent);
    if !parent.is_empty() && !current.is_empty() {
        package.push_str("::");
    }
    package.push_str(current);

    package
}
