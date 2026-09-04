---
name: implement-lua-skills
description: Implements Lua skill registry and prompt dispatch for Mnemo (register_skill, run_skill, list_skills, dispatch). Use when working on skills, dispatch, skills_demo.lua, or when the user asks how prompts map to skills in rag_mnemo.
---

# Implement Lua skills + dispatch

## API (host exposes to Lua)

| Function | Behavior |
|----------|----------|
| `register_skill(name, description, fn)` | Store callback in-memory for this process |
| `run_skill(name, args)` | Call `fn(args)` |
| `list_skills()` | Array of `{ name, description }` |
| `dispatch(text)` | Route user text to a skill |

## `dispatch` rules

1. If text matches `skill:<name>` (optional spaces), run that skill. Unknown name → clear error, do not invent.
2. Else keyword match (case-insensitive), e.g. rag → `explain_rag`, chunk → `explain_chunking`.
3. No match → error listing available skills.

Skills may call `ask(...)` internally. This is **not** an agent loop.

## Scripts

- `scripts/ask.lua` — index + ask only.
- `scripts/skills_demo.lua` — register `explain_rag` / `explain_chunking`, then `run_skill`, `dispatch("skill:...")`, `dispatch` natural language.

Human explanation: `docs/WHAT_IS_A_SKILL.md` section "Skills e o prompt".
