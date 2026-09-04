---
name: follow-spec
description: Treats rag_mnemo Spec Kit files as source of truth. Use when editing this repo, implementing features, changing Lua API, or when the user mentions SDD, spec, CAP-1, Mnemo, or rag_mnemo.
---

# Follow spec (rag_mnemo)

## Before any code change

1. Read `.specify/spec.md` (what to build).
2. Read `.specify/plan.md` (how / tasks).
3. If code and spec disagree, **change the code** or propose a spec update first — never silently violate the spec.

## Scope

- Stack: Rust core + Lua scripts only. No Ruby, no paid APIs.
- Embeddings: deterministic stub hash.
- Binary name: `rag-mnemo`.
- Capabilities: CAP-1 index, CAP-2 ask, CAP-3 Lua API, CAP-4 docs, CAP-5 skills, CAP-6 `dispatch`.

## Non-goals (do not add)

Agent loop, RFC/HITL workflows, OpenAI, web UI, Qdrant.

## Docs for humans

- `README.md`
- `docs/WHAT_IS_A_SKILL.md`
