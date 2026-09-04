/// Resolve um prompt de usuário para o nome de uma skill registrada (sem LLM).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DispatchError {
    UnknownSkill { name: String, available: Vec<String> },
    NoMatch { text: String, available: Vec<String> },
}

impl std::error::Error for DispatchError {}

impl std::fmt::Display for DispatchError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DispatchError::UnknownSkill { name, available } => write!(
                f,
                "skill desconhecida: '{name}'. Skills disponíveis: {}",
                format_available(available)
            ),
            DispatchError::NoMatch { text, available } => write!(
                f,
                "nenhuma skill para: '{text}'. Skills disponíveis: {}",
                format_available(available)
            ),
        }
    }
}

fn format_available(available: &[String]) -> String {
    if available.is_empty() {
        "(nenhuma registrada)".into()
    } else {
        available.join(", ")
    }
}

/// `skill:nome` com espaços opcionais em volta dos dois-pontos.
pub fn parse_skill_colon(text: &str) -> Option<String> {
    let t = text.trim();
    if t.len() < 6 || !t[..5].eq_ignore_ascii_case("skill") {
        return None;
    }
    let rest = t[5..].trim_start();
    let after = rest.strip_prefix(':')?;
    let name = after.trim();
    if name.is_empty() {
        None
    } else {
        Some(name.to_string())
    }
}

fn contains_word(text: &str, word: &str) -> bool {
    text.split(|c: char| !c.is_alphanumeric())
        .any(|t| t == word)
}

/// Palavras-chave → skill convencional (plan: rag, chunking, embedding, skill).
const KEYWORD_SKILLS: &[(&str, &str)] = &[
    ("chunking", "explain_chunking"),
    ("chunk", "explain_chunking"),
    ("embeddings", "explain_embedding"),
    ("embedding", "explain_embedding"),
    ("rag", "explain_rag"),
    ("skill", "explain_skill"),
];

pub fn resolve_dispatch(text: &str, available: &[String]) -> Result<String, DispatchError> {
    let text = text.trim();
    if let Some(name) = parse_skill_colon(text) {
        if available.iter().any(|s| s == &name) {
            return Ok(name);
        }
        return Err(DispatchError::UnknownSkill {
            name,
            available: available.to_vec(),
        });
    }

    let lower = text.to_lowercase();
    for (kw, skill) in KEYWORD_SKILLS {
        if contains_word(&lower, kw) && available.iter().any(|s| s == *skill) {
            return Ok((*skill).to_string());
        }
    }

    for name in available {
        let key = name.rsplit('_').next().unwrap_or(name);
        if key.len() >= 3 && contains_word(&lower, &key.to_lowercase()) {
            return Ok(name.clone());
        }
    }

    Err(DispatchError::NoMatch {
        text: text.to_string(),
        available: available.to_vec(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn skills() -> Vec<String> {
        vec!["explain_rag".into(), "explain_chunking".into()]
    }

    #[test]
    fn explicit_skill_colon() {
        assert_eq!(
            resolve_dispatch("skill:explain_chunking", &skills()).unwrap(),
            "explain_chunking"
        );
        assert_eq!(
            resolve_dispatch("skill: explain_rag", &skills()).unwrap(),
            "explain_rag"
        );
    }

    #[test]
    fn unknown_explicit_skill_errors() {
        let err = resolve_dispatch("skill:nao_existe", &skills()).unwrap_err();
        match err {
            DispatchError::UnknownSkill { name, .. } => assert_eq!(name, "nao_existe"),
            _ => panic!("esperava UnknownSkill"),
        }
    }

    #[test]
    fn keyword_rag() {
        assert_eq!(
            resolve_dispatch("explica o que e RAG", &skills()).unwrap(),
            "explain_rag"
        );
    }

    #[test]
    fn keyword_chunk() {
        assert_eq!(
            resolve_dispatch("como funciona o chunking", &skills()).unwrap(),
            "explain_chunking"
        );
    }

    #[test]
    fn no_match_lists_skills() {
        let err = resolve_dispatch("quero pizza", &skills()).unwrap_err();
        let msg = err.to_string();
        assert!(msg.contains("explain_rag"));
        assert!(msg.contains("pizza"));
    }

    #[test]
    fn does_not_invent_unregistered_keyword_skill() {
        let only_rag = vec!["explain_rag".to_string()];
        let err = resolve_dispatch("fale de chunking", &only_rag).unwrap_err();
        assert!(matches!(err, DispatchError::NoMatch { .. }));
    }
}
