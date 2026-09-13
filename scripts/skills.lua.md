# Script Lua do MemoAssist (`skills.lua`)

O binário `memo-assist` executa [`scripts/skills.lua`](skills.lua).

Igual ao `ask.lua`: o argumento extra escolhe **qual skill** rodar.

```bash
cargo run -- scripts/skills.lua
cargo run -- scripts/skills.lua explain_rag
cargo run -- scripts/skills.lua explain_chunking
```

Without a matching snippet the script prints `Not found.` Unknown skill names are listed; the default skill is `explain_rag`.

API: `index`, `register_skill`, `run_skill`, `list_skills`.
