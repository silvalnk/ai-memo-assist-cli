use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

pub const STORE_PATH: &str = "vector_store.json";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entry {
    pub text: String,
    pub source: String,
    pub embedding: Vec<f32>,
}

#[derive(Debug, Clone)]
pub struct Hit {
    pub text: String,
    pub source: String,
    pub score: f32,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct VectorStore {
    pub entries: Vec<Entry>,
}

impl VectorStore {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    pub fn insert(&mut self, entry: Entry) {
        self.entries.push(entry);
    }

    pub fn search(&self, query: &[f32], k: usize) -> Vec<Hit> {
        let mut hits: Vec<Hit> = self
            .entries
            .iter()
            .map(|e| Hit {
                text: e.text.clone(),
                source: e.source.clone(),
                score: cosine(query, &e.embedding),
            })
            .collect();
        hits.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));
        hits.truncate(k);
        hits
    }

    /// Ranking completo (sem cortar em k) para filtrar só o relacionado.
    pub fn rank_all(&self, query: &[f32]) -> Vec<Hit> {
        self.search(query, self.entries.len())
    }

    pub fn save(&self, path: &Path) -> Result<()> {
        let json = serde_json::to_string_pretty(self).context("serializar vector store")?;
        fs::write(path, json).with_context(|| format!("gravar {}", path.display()))?;
        Ok(())
    }

    pub fn load(path: &Path) -> Result<Self> {
        let raw = fs::read_to_string(path)
            .with_context(|| format!("ler {} — rode index() primeiro", path.display()))?;
        let store: Self = serde_json::from_str(&raw).context("parsear vector store")?;
        Ok(store)
    }
}

/// Score abaixo disso no stub conta como ruído (colisão de hash), não como achado.
pub const MIN_RELATED_SCORE: f32 = 0.15;

/// Mantém só hits relacionados: o melhor tem que passar do piso, e os outros
/// pelo menos metade do melhor (e o mesmo piso).
pub fn related_hits(hits: Vec<Hit>) -> Vec<Hit> {
    let Some(best) = hits.first().map(|h| h.score) else {
        return Vec::new();
    };
    if best < MIN_RELATED_SCORE {
        return Vec::new();
    }
    let floor = (best * 0.5).max(MIN_RELATED_SCORE);
    hits.into_iter().filter(|h| h.score >= floor).collect()
}

/// Cosine similarity. Vetores vazios ou normas zero devolvem 0.0.
pub fn cosine(a: &[f32], b: &[f32]) -> f32 {
    if a.is_empty() || b.is_empty() || a.len() != b.len() {
        return 0.0;
    }
    let mut dot = 0.0f32;
    let mut na = 0.0f32;
    let mut nb = 0.0f32;
    for (x, y) in a.iter().zip(b.iter()) {
        dot += x * y;
        na += x * x;
        nb += y * y;
    }
    if na == 0.0 || nb == 0.0 {
        return 0.0;
    }
    dot / (na.sqrt() * nb.sqrt())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cosine_identical_is_one() {
        let a = vec![1.0, 0.0, 0.0];
        assert!((cosine(&a, &a) - 1.0).abs() < 1e-6);
    }

    #[test]
    fn cosine_orthogonal_is_zero() {
        let a = vec![1.0, 0.0];
        let b = vec![0.0, 1.0];
        assert!(cosine(&a, &b).abs() < 1e-6);
    }

    #[test]
    fn cosine_opposite_is_minus_one() {
        let a = vec![1.0, 0.0];
        let b = vec![-1.0, 0.0];
        assert!((cosine(&a, &b) + 1.0).abs() < 1e-6);
    }

    #[test]
    fn cosine_zero_vector_is_zero() {
        let a = vec![0.0, 0.0];
        let b = vec![1.0, 2.0];
        assert_eq!(cosine(&a, &b), 0.0);
    }

    #[test]
    fn search_ranks_closer_first() {
        let mut store = VectorStore::new();
        store.insert(Entry {
            text: "perto".into(),
            source: "a.md".into(),
            embedding: vec![1.0, 0.0],
        });
        store.insert(Entry {
            text: "longe".into(),
            source: "b.md".into(),
            embedding: vec![0.0, 1.0],
        });
        let hits = store.search(&[1.0, 0.0], 2);
        assert_eq!(hits[0].text, "perto");
        assert!(hits[0].score > hits[1].score);
    }

    fn hit(text: &str, score: f32) -> Hit {
        Hit {
            text: text.into(),
            source: "x.md".into(),
            score,
        }
    }

    #[test]
    fn related_drops_zeros_and_weak_hits() {
        let kept = related_hits(vec![
            hit("melhor", 0.40),
            hit("parecido", 0.28),
            hit("fraco", 0.09),
            hit("zero", 0.0),
        ]);
        let texts: Vec<_> = kept.iter().map(|h| h.text.as_str()).collect();
        assert_eq!(texts, vec!["melhor", "parecido"]);
    }

    #[test]
    fn related_all_zero_is_empty() {
        let kept = related_hits(vec![hit("a", 0.0), hit("b", 0.0)]);
        assert!(kept.is_empty());
    }

    #[test]
    fn related_weak_best_is_empty() {
        let kept = related_hits(vec![hit("colisao", 0.11)]);
        assert!(kept.is_empty());
    }
}
