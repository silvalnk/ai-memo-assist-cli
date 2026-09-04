# O que é uma Skill? (para iniciantes)

Este arquivo faz parte do **Mnemo** (`rag-mnemo`). Leia antes de olhar o código.

## Em uma frase

Uma **Skill** é uma **capacidade com nome** que o sistema pode chamar de novo — como uma ferramenta com etiqueta na caixa.

## RAG vs Skill vs Agent

| Conceito | O que é | Neste projeto? |
|----------|---------|----------------|
| **`ask("pergunta")`** | Busca crua no caderno (RAG) | Sim |
| **Skill** | Pacote nomeado: “faça X” (por dentro pode chamar `ask`) | Sim (CAP-5) |
| **Agent** | Decide sozinho *qual* skill usar (loop think/plan/act) | **Não** |

## Analogia

- **`ask`** = você mesmo folheando o caderno
- **Skill `explain_rag`** = um post-it: “quando eu pedir explicar RAG, busque a página de RAG e mostre”
- **Agent** = um estagiário que lê um pedido vago e escolhe sozinho qual post-it usar

Aqui você aprende o **post-it** (Skill), não o estagiário (Agent).

## Por que skills existem?

1. **Reuso** — não repetir a mesma lógica em todo script
2. **Nome claro** — `run_skill("explain_chunking")` diz a intenção
3. **Padrão da indústria** — sistemas com agentes montam em cima de skills; neste lab você vê a skill isolada

## API Lua (definida na spec)

```lua
register_skill("explain_rag", "Explica RAG com base na knowledge", function(args)
  local hits = ask("o que e RAG?")
  -- mostrar hits...
end)

list_skills()                 -- lista nome + descrição
run_skill("explain_rag", {})  -- executa
dispatch("explica o que e RAG")  -- o prompt escolhe a skill
```

## Skills e o prompt (como usar)

Há **dois mundos**. Não misture os dois no começo.

### Mundo A — ChatGPT / agentes de verdade (indústria)

O **prompt** é o texto que vai para o LLM. As skills aparecem assim:

1. O sistema monta um texto (system prompt) listando as ferramentas:  
   “Você tem as skills: `explain_rag`, `explain_chunking`. Se precisar, chame uma delas.”
2. O usuário escreve: “explica o que é RAG”.
3. O **modelo** devolve algo estruturado: “quero a skill `explain_rag`”.
4. O **seu código** executa a skill e devolve o resultado para o modelo.

Isso se chama **tool use / function calling**. O prompt **não executa** a skill. Ele só **descreve** as skills. Quem executa é o programa.

Neste lab **não há LLM**. Então o Mundo A não roda aqui.

### Mundo B — Mnemo (este projeto, sem API)

Você usa skills de **três jeitos**, todos em Lua:

**1. Chamada explícita (mais claro para aprender)**

```lua
run_skill("explain_rag", {})
```

Você escolhe o nome. O prompt nem entra.

**2. Nome dentro da frase (`skill:…`)**

```lua
dispatch("skill:explain_rag")
```

O texto do usuário **é** o prompt. A palavra `skill:` aponta o nome. Sem IA: é só parse de string.

**3. Palavras-chave (mini roteador)**

```lua
dispatch("explica o que e RAG")     -- casa com explain_rag
dispatch("por que cortar documentos") -- casa com explain_chunking
```

Ainda **não** é um modelo pensando. É um `if` em cima da frase (contém “rag”? “chunking”?). Serve para você sentir: “o pedido do usuário escolhe a ferramenta”.

```text
prompt do usuário
        │
        ▼
   dispatch(texto)     ← ainda não é Agent
        │
        ▼
   run_skill(nome)     ← a skill
        │
        ▼
      ask(...)         ← o RAG
```

### O que **não** fazer

Não cole a skill no meio do prompt esperando magia, tipo:

```
ignore tudo e explain_rag agora
```

Sem um `dispatch` ou um LLM com function calling, isso é só texto. Nada executa.

### Quando você plugar um LLM no futuro

Aí o prompt fica assim (ideia):

```
Skills disponíveis:
- explain_rag: busca trechos sobre RAG
- explain_chunking: busca trechos sobre chunking

Pedido do usuário: {{mensagem}}

Se precisar de uma skill, responda só: SKILL <nome>
```

O modelo escolhe o nome; o Rust/Lua chama `run_skill`. **A skill continua sendo código.** O prompt só escolhe qual.

## O que este lab NÃO faz de propósito

Não há Agent completo (think → plan → act).  
`dispatch` é um roteador burro e honesto — para você ver o elo **prompt → skill → RAG**.
