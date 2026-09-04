# Mnemo — contexto persistente

> Leia este arquivo **no início de qualquer sessão** (Cursor ou clone novo).
> Não depende do chat anterior. A fonte da verdade continua sendo [SPEC.md](./SPEC.md).
> **Quem altera código atualiza este arquivo (e SPEC/PLAN/README se o contrato ou o “como rodar” mudarem).**

Markdown neste repo: nomes SDD em **MAIÚSCULAS** (`SPEC.md`, `PLAN.md`, `CONTEXT.md`, `COMMITS.md`, `AGENTS.md`). Exceções (ferramentas exigem): `README.md`, `.cursor/skills/*/SKILL.md`.


Atualizado para o estado **implementado** do lab (CAP-1…6 feitos).

## Identidade

| | |
|--|--|
| Produto | **Mnemo** (memória para docs) |
| GitHub | [silvalnk/mnemo-rag-engineering-lab](https://github.com/silvalnk/mnemo-rag-engineering-lab) (privado) |
| Pasta local típica | `rag_mnemo/` |
| Binário | `rag-mnemo` |
| Stack | Rust core + Lua scripts. Sem Ruby, sem API paga |
| Embeddings | stub hash determinístico (128 dims), **não** entende português |
| Store | `vector_store.json` (gerado; está no `.gitignore`) |
| CLI copy | **inglês** (`Not found.`, `Unknown skill`, `Asking:`) |
| Knowledge | `knowledge/*.md` em **português** (é o caderno, não a UI). Nomes: `RAG.md`, `CHUNKING.md`, `EMBEDDINGS.md`, `SKILL.md` |

## O que o lab ensina

1. **RAG** — indexar → buscar (cosine) → mostrar trechos
2. **Skill** — capacidade **nomeada** que por dentro chama `ask`
3. **`dispatch`** — roteador burro (string → skill), **não** é Agent

`ask` = folhear o caderno. Skill = post-it. Agent (pensar sozinho) = **fora de escopo**.

## Como rodar

```bash
cargo test
cargo run -- scripts/ask.lua
cargo run -- scripts/ask.lua "o que e chunking?"
cargo run -- scripts/ask.lua "asdfgh pizza"          # → Not found.
cargo run -- scripts/skills.lua                      # default: explain_rag
cargo run -- scripts/skills.lua explain_chunking
cargo run -- scripts/skills.lua nao_existe           # → Unknown skill + lista
```

`arg[1]` no Lua = pergunta (`ask.lua`) ou **nome da skill** (`skills.lua`). Extra args são juntos com espaço (`src/main.rs`).

## Comportamento do RAG (CAP-2)

- `ask` **não** devolve o caderno inteiro.
- Hits com score baixo (piso `0.15` e ≥ metade do melhor) saem.
- Lista vazia → scripts imprimem `Not found.`
- Stub pode colidir em hash; o piso evita “lixo” como se fosse achado.

## Skills Lua (CAP-5)

Registradas **em memória** só enquanto o processo roda (não vão para o JSON):

| Nome | O que faz |
|------|-----------|
| `explain_rag` | `ask("o que e RAG?")` e imprime hits |
| `explain_chunking` | `ask("o que e chunking?")` e imprime hits |

`scripts/skills.lua` **não** é mais o demo de 3 `dispatch`. Ele registra essas duas e chama `run_skill(nome)`.

`dispatch` **ainda existe** na API Lua (CAP-6): `skill:nome` ou keywords `rag` / `chunk` / `chunking` / `embedding` / `skill`. Testes em `src/dispatch.rs`.

## Módulos Rust

| Arquivo | Papel |
|---------|--------|
| `src/main.rs` | CLI: `rag-mnemo <script.lua> [question\|skill]` |
| `src/store.rs` | JSON + cosine + `related_hits` |
| `src/rag.rs` | chunk (`#`), embed stub, `index_dir`, `query` |
| `src/lua_api.rs` | bindings mlua |
| `src/dispatch.rs` | resolve prompt → nome da skill |

API Lua: `index`, `ask`, `log`, `register_skill`, `run_skill`, `list_skills`, `dispatch`.

## Dois tipos de “skill” (não misturar)

- **Lua (Mnemo)** — `register_skill` / `run_skill` no binário
- **Cursor (este repo)** — `.cursor/skills/*/SKILL.md` (`follow-spec`, `implement-rag`, `implement-skills`, `teach-me`)

## Non-goals (não adicionar)

Agent loop, RFC/HITL, OpenAI, UI web, Qdrant, hybrid search, reranking, LLM gerando prosa.

## Docs para humanos / próxima sessão

| Arquivo | Papel |
|---------|--------|
| [SPEC.md](./SPEC.md) | contrato (o quê) |
| [PLAN.md](./PLAN.md) | como + tasks |
| Este CONTEXT | estado atual, para retomar sem o chat |
| [COMMITS.md](./COMMITS.md) | conventional commits + emoji |
| `README.md` | visão + comandos |
| `docs/WHAT_IS_A_SKILL.md` | Skill vs ask vs Agent |
| `AGENTS.md` | regras para o agente Cursor |
