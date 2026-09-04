# Mnemo

> **Memória para os seus documentos.**  
> Lab mínimo de **RAG + Skills** com **Rust + Lua**.  
> 100% offline. Spec-Driven (SDD).

Pasta: `rag-mnemo/` · Marca: **Mnemo** · Binário: `rag-mnemo`

## SDD (comece por aqui)

| Arquivo | Papel |
|---------|--------|
| [`.specify/spec.md`](.specify/spec.md) | **O quê** (fonte da verdade) |
| [`.specify/plan.md`](.specify/plan.md) | **Como** + tasks |
| [`docs/WHAT_IS_A_SKILL.md`](docs/WHAT_IS_A_SKILL.md) | Skill Lua explicada para iniciante |
| [`.cursor/skills/README.md`](.cursor/skills/README.md) | Agent Skills do Cursor (este repo) |
| Este README | Visão geral |

Se código e spec divergirem, a **spec manda**.

## Dois conceitos que você aprende aqui

### 1) RAG
Indexar → buscar trechos parecidos → usar como contexto (aqui: imprimir hits).

### 2) Skill
Capacidade **nomeada** (`explain_rag`) que por dentro chama `ask(...)`.

Como “usar no prompt” neste lab (sem LLM):

1. `run_skill("explain_rag", {})` — você escolhe o nome
2. `dispatch("skill:explain_rag")` — o texto traz o nome
3. `dispatch("explica o que e RAG")` — palavras-chave escolhem a skill

Detalhe: [`docs/WHAT_IS_A_SKILL.md`](docs/WHAT_IS_A_SKILL.md) → seção **Skills e o prompt**.

| | RAG (`ask`) | Skill | Agent (conceito, fora deste lab) |
|--|-------------|-------|----------------------------------|
| Ideia | Busca no caderno | Ferramenta com etiqueta | Programa que escolhe qual ferramenta usar |
| Neste repo | Sim | Sim | Não |

## Como rodar

```bash
cd rag-mnemo
cargo test
cargo run -- scripts/ask.lua                    # pergunta padrão: o que e RAG?
cargo run -- scripts/ask.lua "o que e chunking?"
cargo run -- scripts/ask.lua "asdfgh pizza"
cargo run -- scripts/skills_demo.lua            # RAG + skills
```

Sem trecho relacionado, o script imprime `Não foi encontrado.` Sem argumento, a pergunta padrão continua a da spec.

## Estrutura

```
rag-mnemo/
  .specify/spec.md
  .specify/plan.md
  docs/WHAT_IS_A_SKILL.md
  knowledge/
  scripts/ask.lua
  scripts/skills_demo.lua
  src/                       # Rust: store, rag, lua_api, dispatch
```

## Knowledge base

Textos curtos em `knowledge/` (RAG, chunking, embeddings, skill) — feitos para indexar e perguntar.
