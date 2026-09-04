# Cursor Agent Skills (este repo)

Skills do **agente do Cursor** (arquivos `SKILL.md`). Não confundir com as skills **Lua** do Mnemo (`register_skill` / `run_skill`).

## Como usar no chat

Mencione a skill pelo `name`, ou peça a tarefa que o `description` descreve.

| Skill | Quando |
|-------|--------|
| `conventional-commits` | Pedido de **commit** / commit por arquivo (`✨ feat:` …) |
| `follow-spec` | Qualquer mudança neste projeto (lê CONTEXT + spec; **atualiza os markdowns** no mesmo passo) |
| `implement-rag` | Implementar index/ask/vector store |
| `implement-lua-skills` | Skills Lua + `dispatch` |
| `teach-mnemo` | Explicar conceitos para iniciante |

## Onde ficam

`.cursor/skills/<nome>/SKILL.md` — só este repositório.
