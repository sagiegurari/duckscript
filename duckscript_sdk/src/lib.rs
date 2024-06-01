#![deny(
    future_incompatible,
    keyword_idents,
    let_underscore,
    nonstandard_style,
    unused
)]
#![warn(unknown_lints)]

//! # duckscript_sdk
//!
//! This library contains the standard command library for duckscript.
//!
//! The standard library is not required to use duckscript but it contains common commands and will expand over time.
//!
//! The library is split to 2 parts:
//! * std - The public standard library commands
//! * internal - Used internally by duckscript but also available publicly
//!
//! # Installation
//! See [main duckscript documentation](https://github.com/sagiegurari/duckscript)
//!
//! # Contributing
//! See [contributing guide](https://github.com/sagiegurari/duckscript/blob/master/.github/CONTRIBUTING.md)
//!
//! # License
//! Developed by Sagie Gur-Ari and licensed under the
//! [Apache 2](https://github.com/sagiegurari/duckscript/blob/master/LICENSE) open source license.
//!

pub(crate) mod sdk;
mod types;
mod utils;

#[cfg(test)]
mod test;

use duckscript::types::command::Commands;
use duckscript::types::error::ScriptError;

#[cfg(test)]
#[path = "./lib_test.rs"]
mod lib_test;

static VERSION: &str = env!("CARGO_PKG_VERSION");

/// Returns the library version.
pub fn version() -> String {
    VERSION.to_string()
}

/// Loads all core commands
pub fn load(commands: &mut Commands) -> Result<(), ScriptError> {
    sdk::load(commands)
}
