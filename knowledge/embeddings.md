# Embeddings

Um embedding é uma lista de números que representa um texto.

Textos com significado parecido deveriam ter números parecidos. Assim o computador pergunta: “qual trecho está mais perto desta pergunta?”

A métrica neste lab é cosine similarity:

- perto de 1 = muito parecido
- perto de 0 = pouco relacionado

Importante: no Mnemo o embedding é um STUB (hash). Ele não “entende” português de verdade. Serve para você ver o pipeline. Em produção, um modelo de embedding gera números com significado real.
