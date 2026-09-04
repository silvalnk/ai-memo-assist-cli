use crate::dispatch::resolve_dispatch;
use crate::rag::{index_dir, query, DEFAULT_TOP_K};
use mlua::{Function, Lua, RegistryKey, Result as LuaResult, Value};
use std::cell::RefCell;
use std::collections::HashMap;
use std::path::Path;

struct Skill {
    description: String,
    func: RegistryKey,
}

type Registry = RefCell<HashMap<String, Skill>>;

pub fn create_lua() -> LuaResult<Lua> {
    let lua = Lua::new();
    lua.set_app_data(Registry::new(HashMap::new()));

    let globals = lua.globals();

    globals.set(
        "log",
        lua.create_function(|_, msg: String| {
            println!("{msg}");
            Ok(())
        })?,
    )?;

    globals.set(
        "index",
        lua.create_function(|_, dir: String| {
            let n = index_dir(Path::new(&dir)).map_err(mlua::Error::external)?;
            Ok(n)
        })?,
    )?;

    globals.set(
        "ask",
        lua.create_function(|lua, question: String| {
            let hits = query(&question, DEFAULT_TOP_K).map_err(mlua::Error::external)?;
            let table = lua.create_table()?;
            for (i, hit) in hits.iter().enumerate() {
                let row = lua.create_table()?;
                row.set("text", hit.text.as_str())?;
                row.set("score", hit.score)?;
                row.set("source", hit.source.as_str())?;
                table.set(i + 1, row)?;
            }
            Ok(table)
        })?,
    )?;

    globals.set(
        "register_skill",
        lua.create_function(
            |lua, (name, description, func): (String, String, Function)| {
                let key = lua.create_registry_value(func)?;
                let registry = lua
                    .app_data_ref::<Registry>()
                    .expect("skill registry");
                registry.borrow_mut().insert(
                    name,
                    Skill {
                        description,
                        func: key,
                    },
                );
                Ok(())
            },
        )?,
    )?;

    globals.set(
        "list_skills",
        lua.create_function(|lua, ()| {
            let registry = lua
                .app_data_ref::<Registry>()
                .expect("skill registry");
            let mut names: Vec<(String, String)> = registry
                .borrow()
                .iter()
                .map(|(n, s)| (n.clone(), s.description.clone()))
                .collect();
            drop(registry);
            names.sort_by(|a, b| a.0.cmp(&b.0));

            let table = lua.create_table()?;
            for (i, (name, description)) in names.iter().enumerate() {
                let row = lua.create_table()?;
                row.set("name", name.as_str())?;
                row.set("description", description.as_str())?;
                table.set(i + 1, row)?;
            }
            Ok(table)
        })?,
    )?;

    globals.set(
        "run_skill",
        lua.create_function(|lua, (name, args): (String, Value)| run_skill(lua, &name, args))?,
    )?;

    globals.set(
        "dispatch",
        lua.create_function(|lua, text: String| {
            let registry = lua
                .app_data_ref::<Registry>()
                .expect("skill registry");
            let available: Vec<String> = registry.borrow().keys().cloned().collect();
            drop(registry);

            let name = resolve_dispatch(&text, &available).map_err(mlua::Error::external)?;
            let empty = Value::Table(lua.create_table()?);
            run_skill(lua, &name, empty)
        })?,
    )?;

    Ok(lua)
}

/// `arg[0]` = script; `arg[1]` = pergunta (args extras juntos, para busca dinâmica).
pub fn set_script_args(lua: &Lua, script: &str, extra: &[String]) -> LuaResult<()> {
    let arg = lua.create_table()?;
    arg.set(0, script)?;
    if !extra.is_empty() {
        arg.set(1, extra.join(" "))?;
    }
    lua.globals().set("arg", arg)?;
    Ok(())
}

fn run_skill(lua: &Lua, name: &str, args: Value) -> mlua::Result<Value> {
    let func = {
        let registry = lua
            .app_data_ref::<Registry>()
            .expect("skill registry");
        let borrow = registry.borrow();
        let skill = borrow.get(name).ok_or_else(|| {
            mlua::Error::external(format!(
                "skill '{name}' não registrada. Use list_skills()."
            ))
        })?;
        lua.registry_value::<Function>(&skill.func)?
    };

    let args = match args {
        Value::Nil => Value::Table(lua.create_table()?),
        other => other,
    };
    func.call::<Value>(args)
}

/// Só para testes que precisam inspecionar o registry sem Lua.
#[allow(dead_code)]
pub fn registered_names(lua: &Lua) -> Vec<String> {
    let registry = lua.app_data_ref::<Registry>().expect("skill registry");
    let mut names: Vec<String> = registry.borrow().keys().cloned().collect();
    names.sort();
    names
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn register_list_and_run() {
        let lua = create_lua().unwrap();
        lua.load(
            r#"
            register_skill("echo", "devolve o nome", function(args)
              return args.x
            end)
            "#,
        )
        .exec()
        .unwrap();
        assert_eq!(registered_names(&lua), vec!["echo".to_string()]);

        let out: String = lua
            .load(r#"return run_skill("echo", { x = "ok" })"#)
            .eval()
            .unwrap();
        assert_eq!(out, "ok");

        let listed: usize = lua
            .load(r#"return #list_skills()"#)
            .eval()
            .unwrap();
        assert_eq!(listed, 1);
    }

    #[test]
    fn dispatch_unknown_is_clear_error() {
        let lua = create_lua().unwrap();
        lua.load(
            r#"
            register_skill("explain_rag", "x", function(args) end)
            "#,
        )
        .exec()
        .unwrap();
        let err = lua
            .load(r#"dispatch("skill:nao_existe")"#)
            .exec()
            .unwrap_err();
        let msg = err.to_string();
        assert!(msg.contains("nao_existe"), "{msg}");
        assert!(msg.contains("explain_rag"), "{msg}");
    }
}
