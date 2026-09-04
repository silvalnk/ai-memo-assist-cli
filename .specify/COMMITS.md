# Conventional commits + emoji

Fonte da verdade para **mensagens de git** neste repo.  
Skill do agente: `.cursor/skills/conventional-commits/SKILL.md`.  
Na próxima sessão: `/conventional-commits` ou “faça o commit”.

## Format

```
<emoji> <type>: <subject>

[optional body — why, not a file dump]
```

- Language: **English**
- Subject: imperative, no trailing period, ~72 chars
- One logical change per commit **unless** the user asks for one commit **per file**

## Types and emoji

| Type | Emoji | Use when |
|------|-------|----------|
| `feat` | ✨ | New behavior (CLI, Lua API, RAG, skills) |
| `fix` | 🐛 | Bug fix |
| `docs` | 📝 | spec, CONTEXT, README, knowledge, `*.lua.md` |
| `chore` | 🙈 | gitignore, tooling |
| `chore` | 🔒 | `Cargo.lock` |
| `chore` | 🤖 | `.cursor/skills/` |
| `chore` | 📦 | `Cargo.toml` only (deps/manifest) |
| `refactor` | ♻️ | Same behavior, clearer code |
| `test` | ✅ | Tests only |
| `perf` | ⚡ | Performance |
| `style` | 💄 | Formatting only |

If one commit touches mixed concerns, pick the **primary** type (usually `feat` or `fix`) and mention docs in the body. Prefer updating markdown **in the same commit** as the behavior (CAP-4).

## Examples (this repo)

```
✨ feat: run skills by CLI name with English CLI copy
📝 docs: add Mnemo product spec
🙈 chore: ignore build artifacts and vector store
🤖 chore: add follow-spec agent skill
🔒 chore: lock Rust dependencies
```

## Never

- Commit `target/`, `vector_store.json`, `.env`, secrets
- `--no-verify` / skip hooks unless the user asks
- Force-push `main`
- Push unless the user asks
- Amend a commit that is already on the remote (unless the user explicitly asks)

## Per-file mode

If the user says “commit each file” / “um commit por arquivo”:

1. Stage **one path**
2. Commit with the table above
3. Repeat

Otherwise: **one commit** for the whole related change.

## After commit

- `git status` must be clean for the intended files
- Push **only** if asked (`git push -u origin HEAD` as needed)
