# Mnemo / rag-mnemo — Spec (Spec Kit)

> Spec-Driven Development: este arquivo é a **fonte da verdade**.
> Pasta do projeto: `rag-mnemo/`. Produto: **Mnemo** (memória para seus docs).

## Intent

Iniciantes em engenharia de IA precisam **ver e tocar**:

1. o pipeline de **RAG** (indexar → buscar → usar trechos), e
2. o conceito de **Skill** (capacidade nomeada e reutilizável),

sem o ruído de um Agent completo, workflows ou APIs pagas.

O Mnemo é um laboratório mínimo (**Rust + Lua**): indexa markdown local, busca por similaridade e permite **registrar e executar skills em Lua** que usam o RAG por baixo — como ferramentas rotuladas na caixa de ferramentas.

**Nome:** *Mnemo* (Mnemosyne / memória). Pasta: `rag-mnemo`.

## Capabilities

- **CAP-1** — Indexar markdown de uma pasta (`knowledge/`)
  - **intent:** Ler `.md`, chunking, embeddings stub, gravar vector store.
  - **success:** Após `index(...)`, existir `vector_store.json` com N > 0 entradas.

- **CAP-2** — Buscar trechos por pergunta
  - **intent:** Retornar top-k chunks **relacionados** com `score`, `text`, `source` (cosine similarity). Não devolver o caderno inteiro nem hits com score ≤ 0.
  - **success:** `ask("o que e RAG?")` devolve só trechos relevantes; pergunta sem overlap (ou só ruído do stub) devolve lista vazia. Scripts imprimem `Not found.` quando a lista é vazia.
  - **CLI:** `rag-mnemo scripts/ask.lua` usa a pergunta padrão; `rag-mnemo scripts/ask.lua "o que e chunking?"` busca de forma dinâmica.

- **CAP-3** — Script Lua como interface
  - **intent:** API Lua: `index`, `ask`, `log`, `register_skill`, `run_skill`, `list_skills`, `dispatch`.
  - **success:** `cargo run -- scripts/ask.lua` e `cargo run -- scripts/skills.lua` funcionam.

- **CAP-4** — Documentação SDD + README didático
  - **intent:** Spec, plan e README (RAG + Skill) para iniciante.
  - **success:** Leitor explica as 3 etapas do RAG e a diferença Skill vs `ask` direto.

- **CAP-5** — Mini sistema de Skills (sem Agent completo)
  - **intent:** Skill = capacidade **nomeada** + descrição + função Lua. O usuário registra skills que por dentro chamam `ask` (ou lógica fixa). Assim se aprende o padrão “skill = ferramenta reutilizável” usado em agentes — **sem** o loop think/plan/act de um Agent.
  - **success:**
    1. `register_skill("explain_rag", "Explica RAG usando a knowledge base", fn)`
    2. `run_skill("explain_rag", {})` imprime trechos via RAG
    3. `list_skills()` lista nome + descrição
  - **CLI:** `rag-mnemo scripts/skills.lua` usa `explain_rag`; `rag-mnemo scripts/skills.lua explain_chunking` executa pelo nome e imprime o resultado. Nome desconhecido → erro claro (não inventa skill).
  - **doc:** `docs/WHAT_IS_A_SKILL.md` + `knowledge/skill.md`

- **CAP-6** — Ligar um **prompt do usuário** a uma skill (sem LLM)
  - **intent:** Mostrar o padrão da indústria “o texto do usuário escolhe a ferramenta”, de forma didática. Sem modelo pago: `dispatch(texto)` lê a frase, escolhe uma skill pelo **nome** ou por **palavras-chave**, e chama `run_skill`.
  - **success:** `dispatch("explica o que e RAG")` executa `explain_rag`; `dispatch("skill:explain_chunking")` executa pelo nome explícito; frase desconhecida retorna erro claro (não inventa skill).
  - **doc:** seção “Skills e o prompt” em `docs/WHAT_IS_A_SKILL.md`

## Boundaries / Constraints

- Stack: **Rust** (core) + **Lua** (scripts) — sem Ruby
- Sem OpenAI / Anthropic / chave de API
- Embeddings: **stub determinístico** (hash)
- Persistência: `vector_store.json`
- Skills vivem no runtime Lua (registro em memória por execução) — sem Agent Orchestrator
- Binário CLI: `rag-mnemo` (alinhado ao nome da pasta)

## Non-goals

- **Agent loop** completo (think → plan → act → observe), multi-agent, orchestrator inteligente
- Workflows, RFC, HITL
- LLM real gerando prosa (Ollama = futuro)
- Hybrid search, reranking, UI web

> Skills **sim**. Agent completo (loop think/plan/act, orquestração automática) **não** — fora do escopo deste lab.

## Success Signal

Alguém clona `rag-mnemo`, lê a spec, roda:

1. `scripts/ask.lua` → vê hits do RAG
2. `scripts/skills.lua [nome]` → vê o resultado da skill (`explain_rag` por padrão)

e consegue dizer: “`ask` é a busca crua; a **skill** é um pacote nomeado; o **prompt** só escolhe qual skill rodar.”

## Assumptions

- Rust (`cargo`) instalado
- `knowledge/` tem textos curtos (RAG, chunking, embeddings, skill)

## Open Questions

- [Resolvido] Pasta = `rag-mnemo`, binário = `rag-mnemo`, marca = Mnemo
- Futuro: Ollama embeddings sem mudar API Lua das skills
