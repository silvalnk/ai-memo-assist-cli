---
name: teach-mnemo
description: Explains Mnemo RAG and Skills to beginners in simple Portuguese. Use when the user asks what RAG, skill, dispatch, embedding, or chunking is, or how to use skills in a prompt in this lab.
---

# Teach Mnemo (beginner)

Answer in Portuguese. Keep analogies. Do not mix in other repos.

## Sources (read if needed)

- `README.md`
- `docs/WHAT_IS_A_SKILL.md` — RAG vs Skill vs Agent; prompt → dispatch
- `.specify/spec.md` — official capabilities
- `knowledge/*.md` — short study texts

## Teaching map

- **ask** = raw search in the notebook (RAG).
- **Skill** = named tool; inside it can call `ask`.
- **dispatch(prompt)** = dumb router from user sentence to a skill (no LLM).
- **Agent** = out of scope here.

If they ask "how do I use skills in the prompt?": three ways — `run_skill`, `dispatch("skill:nome")`, `dispatch("explica RAG")`. A skill name dumped in prose does nothing without dispatch or an LLM.
