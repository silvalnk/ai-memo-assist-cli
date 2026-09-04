use crate::store::{Entry, Hit, VectorStore, STORE_PATH};
use anyhow::{bail, Context, Result};
use std::collections::hash_map::DefaultHasher;
use std::fs;
use std::hash::{Hash, Hasher};
use std::path::Path;

pub const EMBED_DIM: usize = 128;
pub const DEFAULT_TOP_K: usize = 5;

/// Embedding stub: bag-of-words com hash determinístico (sem rede, sem modelo).
pub fn embed(text: &str) -> Vec<f32> {
    let mut v = vec![0.0f32; EMBED_DIM];
    for token in tokenize(text) {
        let idx = hash_token(&token) % EMBED_DIM;
        v[idx] += 1.0;
    }
    let norm: f32 = v.iter().map(|x| x * x).sum::<f32>().sqrt();
    if norm > 0.0 {
        for x in &mut v {
            *x /= norm;
        }
    }
    v
}

fn tokenize(text: &str) -> Vec<String> {
    let folded: String = text
        .chars()
        .map(|c| {
            if c.is_alphanumeric() {
                c.to_ascii_lowercase()
            } else {
                ' '
            }
        })
        .collect();
    folded
        .split_whitespace()
        .filter(|t| t.len() >= 2 && !is_stopword(t))
        .map(|s| s.to_string())
        .collect()
}

fn is_stopword(token: &str) -> bool {
    matches!(
        token,
        "que" | "um" | "uma" | "de" | "do" | "da" | "dos" | "das" | "em" | "no" | "na"
            | "os" | "as" | "para" | "com" | "por" | "como" | "e" | "o" | "a"
    )
}

fn hash_token(token: &str) -> usize {
    let mut hasher = DefaultHasher::new();
    token.hash(&mut hasher);
    hasher.finish() as usize
}

/// Corta markdown por títulos (`#`). Sem títulos, cai em parágrafos.
pub fn chunk_markdown(content: &str) -> Vec<String> {
    let mut chunks = Vec::new();
    let mut current = String::new();
    let mut saw_heading = false;

    for line in content.lines() {
        if line.starts_with('#') {
            saw_heading = true;
            if !current.trim().is_empty() {
                chunks.push(current.trim().to_string());
                current.clear();
            }
        }
        current.push_str(line);
        current.push('\n');
    }
    if !current.trim().is_empty() {
        chunks.push(current.trim().to_string());
    }

    if !saw_heading {
        chunks.clear();
        for para in content.split("\n\n") {
            let t = para.trim();
            if !t.is_empty() {
                chunks.push(t.to_string());
            }
        }
    }

    chunks.retain(|c| !c.trim().is_empty());
    chunks
}

pub fn index_dir(dir: &Path) -> Result<usize> {
    if !dir.is_dir() {
        bail!("{} não é um diretório", dir.display());
    }

    let mut store = VectorStore::new();
    let mut files: Vec<_> = fs::read_dir(dir)
        .with_context(|| format!("ler {}", dir.display()))?
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| p.extension().and_then(|s| s.to_str()) == Some("md"))
        .collect();
    files.sort();

    for path in files {
        let content = fs::read_to_string(&path)
            .with_context(|| format!("ler {}", path.display()))?;
        let source = path.to_string_lossy().replace('\\', "/");
        for chunk in chunk_markdown(&content) {
            store.insert(Entry {
                embedding: embed(&chunk),
                text: chunk,
                source: source.clone(),
            });
        }
    }

    if store.entries.is_empty() {
        bail!("nenhum chunk gerado em {}", dir.display());
    }

    store.save(Path::new(STORE_PATH))?;
    Ok(store.entries.len())
}

pub fn query(question: &str, k: usize) -> Result<Vec<Hit>> {
    let store = VectorStore::load(Path::new(STORE_PATH))?;
    if store.entries.is_empty() {
        bail!("vector store vazio — rode index() primeiro");
    }
    let ranked = store.rank_all(&embed(question));
    let mut related = crate::store::related_hits(ranked);
    if related.len() > k {
        related.truncate(k);
    }
    Ok(related)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn embed_is_deterministic() {
        let a = embed("o que e RAG?");
        let b = embed("o que e RAG?");
        assert_eq!(a, b);
        assert_eq!(a.len(), EMBED_DIM);
    }

    #[test]
    fn embed_same_tokens_match() {
        let q = embed("o que e RAG");
        let d = embed("# O que e RAG?\n\nRAG significa Retrieval-Augmented Generation.");
        let other = embed("chunking corta documentos em pedacos menores");
        let sim_rag = crate::store::cosine(&q, &d);
        let sim_other = crate::store::cosine(&q, &other);
        assert!(sim_rag > sim_other);
    }

    #[test]
    fn chunk_splits_on_headings() {
        let text = "# Um\n\nalfa\n\n# Dois\n\nbeta\n";
        let chunks = chunk_markdown(text);
        assert_eq!(chunks.len(), 2);
        assert!(chunks[0].contains("Um"));
        assert!(chunks[1].contains("Dois"));
    }

    #[test]
    fn chunk_falls_back_to_paragraphs() {
        let text = "primeiro paragrafo\n\nsegundo paragrafo";
        let chunks = chunk_markdown(text);
        assert_eq!(chunks.len(), 2);
    }
}
