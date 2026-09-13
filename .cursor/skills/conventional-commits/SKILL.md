---
name: conventional-commits
description: Create git commits for ai-memo-assist-cli using conventional commits plus emoji, in English. Use when the user asks to commit, conventional commit, commit message, git commit, or commit each file.
---

# Conventional commits + emoji

Before committing, read [`.specify/COMMITS.md`](../../.specify/COMMITS.md) and follow it exactly.

## Format

```
<emoji> <type>: <subject>
```

English. Imperative subject. Optional body = **why**.

| Type | Emoji |
|------|-------|
| feat | ✨ |
| fix | 🐛 |
| docs | 📝 |
| chore (gitignore) | 🙈 |
| chore (lockfile) | 🔒 |
| chore (Cursor skills) | 🤖 |
| chore (Cargo.toml) | 📦 |
| refactor | ♻️ |
| test | ✅ |

Default: **one commit** for the related change (include matching markdown in that commit).

If the user asks for **one commit per file**: stage and commit each path separately using this table.

## Safety (this repo)

- Do not commit `target/` or `vector_store.json`
- Do not skip hooks
- Do not push unless asked
- Never update git config
- HEREDOC for the message (user git rules)

Also follow the user's global git protocol: `git status`, `git diff`, `git log` before committing.
