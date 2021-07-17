mod base64;
mod base64_decode;
mod base64_encode;
pub(crate) mod bytes_to_string;
mod camelcase;
mod concat;
mod contains;
mod ends_with;
pub(crate) mod equals;
mod indexof;
mod is_empty;
mod kebabcase;
mod last_indexof;
mod length;
mod lowercase;
mod replace;
mod snakecase;
mod split;
mod starts_with;
pub(crate) mod string_to_bytes;
mod substring;
mod trim;
mod trim_end;
mod trim_start;
mod uppercase;

use crate::utils::pckg;
use duckscript::types::command::Commands;
use duckscript::types::error::ScriptError;

static PACKAGE: &str = "string";

pub(crate) fn load(commands: &mut Commands, parent: &str) -> Result<(), ScriptError> {
    let package = pckg::concat(parent, PACKAGE);

    commands.set(base64::create(&package)?)?;
    commands.set(base64_decode::create(&package))?;
    commands.set(base64_encode::create(&package))?;
    commands.set(bytes_to_string::create(&package))?;
    commands.set(camelcase::create(&package))?;
    commands.set(concat::create(&package)?)?;
    commands.set(contains::create(&package))?;
    commands.set(ends_with::create(&package))?;
    commands.set(equals::create(&package))?;
    commands.set(indexof::create(&package))?;
    commands.set(is_empty::create(&package))?;
    commands.set(kebabcase::create(&package))?;
    commands.set(last_indexof::create(&package))?;
    commands.set(length::create(&package))?;
    commands.set(lowercase::create(&package))?;
    commands.set(replace::create(&package))?;
    commands.set(snakecase::create(&package))?;
    commands.set(split::create(&package))?;
    commands.set(starts_with::create(&package))?;
    commands.set(string_to_bytes::create(&package))?;
    commands.set(substring::create(&package))?;
    commands.set(trim::create(&package))?;
    commands.set(trim_start::create(&package))?;
    commands.set(trim_end::create(&package))?;
    commands.set(uppercase::create(&package))?;

    Ok(())
}
