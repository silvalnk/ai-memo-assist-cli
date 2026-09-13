# O que é uma Skill?

Uma Skill (habilidade) é uma capacidade **com nome** que um sistema de IA pode reutilizar.

Exemplo: a skill `explain_rag` significa “buscar nos documentos o que é RAG e mostrar os trechos”.

Diferença importante:

- **ask** = busca direta no banco de vetores (RAG)
- **Skill** = um pacote nomeado que pode usar o ask por dentro
- **Agent** = quem decide sozinho qual skill usar (não faz parte do MemoAssist)

Skills existem para organizar ferramentas. No MemoAssist você registra e executa skills em Lua para aprender esse padrão.
