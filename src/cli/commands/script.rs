//! Lua script execution command

use std::path::PathBuf;

use crate::error::Result;
use crate::lua::executor::ScriptEngine;
use crate::lua::stdlib::LuaStdLib;

pub async fn run_lua_script(path: PathBuf, args: Vec<String>) -> Result<()> {
    // 1. Create script engine
    let engine = ScriptEngine::new()?;

    // 2. Register all available APIs
    engine.bindings.register_all_apis()?;

    // 3. Register stdlib utilities
    let lua = engine.bindings.lua();
    LuaStdLib::register_all_on(lua)?;

    // 4. Read script file
    let script_content = std::fs::read_to_string(&path).map_err(crate::error::SerialError::Io)?;

    // 5. Execute script with arguments
    if args.is_empty() {
        engine.execute_file(&path)?;
    } else {
        tracing::info!("Executing script with arguments: {:?}", args);
        engine.execute_with_args(&script_content, args)?;
    }

    Ok(())
}
