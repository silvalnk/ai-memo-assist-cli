mod dispatch;
mod lua_api;
mod rag;
mod store;

use anyhow::{bail, Context, Result};

fn main() -> Result<()> {
    let mut args = std::env::args().skip(1);
    let Some(script) = args.next() else {
        bail!("uso: rag-mnemo <script.lua> [pergunta]");
    };
    let extra: Vec<String> = args.collect();

    let source = std::fs::read_to_string(&script)
        .with_context(|| format!("ler script {script}"))?;
    let lua = lua_api::create_lua().map_err(|e| anyhow::anyhow!("{e}"))?;
    lua_api::set_script_args(&lua, &script, &extra).map_err(|e| anyhow::anyhow!("{e}"))?;
    lua.load(&source)
        .set_name(script.as_str())
        .exec()
        .map_err(|e| anyhow::anyhow!("executar {script}: {e}"))?;
    Ok(())
}
