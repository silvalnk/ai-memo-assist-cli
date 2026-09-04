# skills_demo.lua (especificação do script)

Quando o código Rust existir, este conteúdo vira `scripts/skills_demo.lua`.

Objetivo (CAP-5): mostrar **Skill** usando RAG por baixo.

```lua
log("=== Mnemo skills demo ===")
index("knowledge/")

register_skill(
  "explain_rag",
  "Busca na knowledge o que e RAG e imprime os trechos",
  function(args)
    local hits = ask("o que e RAG?")
    log("Skill explain_rag: " .. tostring(#hits) .. " hits")
    for i, hit in ipairs(hits) do
      print(string.format("[%d] %.4f %s", i, hit.score, hit.source))
      print(hit.text)
      print("---")
    end
  end
)

register_skill(
  "explain_chunking",
  "Busca na knowledge o que e chunking",
  function(args)
    local hits = ask("o que e chunking?")
    for i, hit in ipairs(hits) do
      print(string.format("[%d] %.4f %s", i, hit.score, hit.source))
      print(hit.text)
    end
  end
)

log("Skills registradas:")
for _, s in ipairs(list_skills()) do
  print(" - " .. s.name .. ": " .. s.description)
end

log("1) Chamada explícita: run_skill")
run_skill("explain_rag", {})

log("2) Prompt com nome: dispatch('skill:explain_chunking')")
dispatch("skill:explain_chunking")

log("3) Prompt em linguagem natural: dispatch('explica o que e RAG')")
dispatch("explica o que e RAG")
```

Smoke: `cargo run -- scripts/skills_demo.lua`
