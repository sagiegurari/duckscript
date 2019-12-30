mod sdkdocs;

use duckscript::types::command::Commands;
use duckscript::types::error::ScriptError;

static PACKAGE: &str = "internal";

pub(crate) fn load(commands: &mut Commands) -> Result<(), ScriptError> {
    commands.set(sdkdocs::create(PACKAGE))?;

    Ok(())
}
