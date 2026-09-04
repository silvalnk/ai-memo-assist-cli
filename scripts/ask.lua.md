# Script Lua do Mnemo (`ask.lua`)

O binário `rag-mnemo` executa [`scripts/ask.lua`](ask.lua).

```lua
log("Mnemo: indexando knowledge/")
index("knowledge/")

local pergunta = (arg and arg[1] and arg[1] ~= "") and arg[1] or "o que e RAG?"
local hits = ask(pergunta)
if #hits == 0 then
  print("Não foi encontrado.")
else
  -- só trechos relacionados (filtro no Rust)
end
```

API Lua (definida na spec / plan):

| Função | Capacidade |
|--------|------------|
| `index(dir)` | CAP-1 |
| `ask(question)` | CAP-2 — só hits relacionados; vazio se nada achar |
| `log(msg)` | utilitário |

```bash
cargo run -- scripts/ask.lua
cargo run -- scripts/ask.lua "o que e chunking?"
```
