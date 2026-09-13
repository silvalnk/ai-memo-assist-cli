# Chunking

Chunking é cortar um documento grande em pedaços menores.

Por quê? Porque a busca funciona melhor com trechos focados. Um livro inteiro como um único bloco é difícil de “apontar”. Um parágrafo sobre um assunto só é mais fácil de recuperar.

Exemplo: este arquivo é um chunk pequeno de propósito.

No MemoAssist, o Rust corta por títulos markdown (`#`) ou por parágrafos.

Pedaço demais pequeno perde contexto. Pedaço demais grande mistura assuntos. O equilíbrio é parte do desenho do RAG.
