---
name: follow-spec
description: Treats rag_mnemo Spec Kit files as source of truth. Use when editing this repo, implementing features, changing Lua API, or when the user mentions SDD, spec, CAP-1, Mnemo, or rag_mnemo.
---

# Follow spec (rag_mnemo)

## Before any code change

1. Read `.specify/CONTEXT.md` (current state — survives a new Cursor session).
2. Read `.specify/SPEC.md` (what to build).
3. Read `.specify/PLAN.md` (how / tasks).
4. If code and spec disagree, **change the code** or propose a spec update first — never silently violate the spec.
5. **After any behavior change, update the markdowns in the same turn.** Do not leave docs stale.

Also read `AGENTS.md` at the repo root.

## Keep markdowns in sync (mandatory)

When you change CLI, Lua API, RAG, skills, file names, or messages, update **in the same task**:

| File | Update when |
|------|-------------|
| `.specify/SPEC.md` | Contract changed (CAPs, success, CLI, boundaries) |
| `.specify/PLAN.md` | Architecture, modules, or tasks changed |
| `.specify/CONTEXT.md` | **Always** — this is the live snapshot |
| `AGENTS.md` | Commands, API, or agent rules changed |
| `README.md` | How to run / structure / GitHub |
| `docs/WHAT_IS_A_SKILL.md` | How skills/`dispatch` are used |
| `scripts/*.lua.md` | Matching script behavior |
| `.specify/COMMITS.md` | Only if the commit convention itself changes |
| `.cursor/skills/*/SKILL.md` | Agent recipes no longer match the code |

A change is **not done** until CONTEXT.md matches the code. If the user did not ask for a spec change but behavior changed, still update spec (or propose the spec edit first if it would violate a CAP).


## Scope

- Stack: Rust core + Lua scripts only. No Ruby, no paid APIs.
- Embeddings: deterministic stub hash.
- Binary name: `rag-mnemo`.
- Capabilities: CAP-1 index, CAP-2 ask, CAP-3 Lua API, CAP-4 docs, CAP-5 skills, CAP-6 `dispatch`.
- CLI: `rag-mnemo <script.lua> [question|skill]`. Scripts print English (`Not found.`, `Unknown skill`).
- Skills script: `scripts/skills.lua [name]` (default `explain_rag`), not `skills_demo.lua`.

## Non-goals (do not add)

Agent loop, RFC/HITL workflows, OpenAI, web UI, Qdrant.

## Docs for humans

- `README.md`
- `docs/WHAT_IS_A_SKILL.md`
- `.specify/CONTEXT.md`
- `.specify/COMMITS.md`

