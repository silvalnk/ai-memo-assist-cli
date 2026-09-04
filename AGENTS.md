# AGENTS.md

This file is the **session-independent** brief for Cursor (and any coding agent). Chat history is optional; these files are not.

## Always

1. Read [`.specify/CONTEXT.md`](.specify/CONTEXT.md) (current product state).
2. Read [`.specify/spec.md`](.specify/spec.md) (source of truth).
3. Read [`.specify/plan.md`](.specify/plan.md) before changing how things are built.
4. If code and spec disagree, **change the code** or propose a spec update first. Never silently violate the spec.
5. Follow [`.cursor/skills/follow-spec/SKILL.md`](.cursor/skills/follow-spec/SKILL.md) on any code change.
6. **Same turn as the code change:** update `.specify/CONTEXT.md` and any of spec / plan / README / `docs/WHAT_IS_A_SKILL.md` / agent skills that would otherwise lie. Docs lagging behind code is a spec violation (CAP-4).

7. When the user asks to **commit**, follow [`.specify/COMMITS.md`](.specify/COMMITS.md) and [`.cursor/skills/conventional-commits/SKILL.md`](.cursor/skills/conventional-commits/SKILL.md) (`✨ feat:` / `📝 docs:` / …, English).

## Keep docs in sync

Behavior change → markdown change. Minimum: **CONTEXT.md**. Contract change → **spec.md** too. How-to for humans → **README.md**.


## Product (one paragraph)

**Mnemo** (`rag-mnemo`) is an offline Rust + Lua lab: index local `knowledge/*.md`, stub-hash embeddings, cosine search, in-memory Lua skills. Binary: `rag-mnemo`. Repo: `silvalnk/mnemo-rag-engineering-lab`.

## Commands

```bash
cargo test
cargo run -- scripts/ask.lua [question]
cargo run -- scripts/skills.lua [skill_name]
```

Defaults: question `o que e RAG?`; skill `explain_rag`. Empty RAG hits print `Not found.` Unknown skill names are listed, never invented. CLI strings are **English**; knowledge files stay Portuguese.

## Do not add

Agent loop, RFC/HITL, OpenAI/Anthropic, web UI, Qdrant, paid APIs, Ruby.

## Lua API

`index`, `ask`, `log`, `register_skill`, `run_skill`, `list_skills`, `dispatch`.

`dispatch` is a dumb string router (CAP-6), not an agent. The `skills.lua` script runs skills **by CLI name** (`run_skill`), not the old three-step demo.

## Human docs

- `README.md`
- `docs/WHAT_IS_A_SKILL.md`
- `.specify/COMMITS.md` — git messages (emoji conventional commits)

