---
name: implement-rag
description: Implements the Mnemo RAG pipeline in Rust (chunk, stub embeddings, cosine search, vector_store.json). Use when adding src/, Cargo.toml, index/ask, or when the user asks to implement RAG, vector store, or chunking in rag_mnemo.
---

# Implement RAG (rag_mnemo)

## Contract

- `index(dir)`: read `*.md`, chunk (prefer markdown headings), stub embedding, save `vector_store.json`.
- `ask(question)`: return list of `{ text, score, source }` (top-k cosine similarity).
- No network. Stub embedding must be deterministic.

## Layout

```
src/main.rs      # CLI: rag-mnemo <script.lua>
src/store.rs     # insert/search/save/load + cosine tests
src/rag.rs       # chunk + embed stub + index_dir + query
src/lua_api.rs   # mlua bindings
```

## Done when

- `cargo test` passes (include cosine tests).
- `cargo run -- scripts/ask.lua` prints ranked hits from `knowledge/`.
