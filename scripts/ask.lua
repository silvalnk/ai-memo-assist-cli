-- Mnemo — volante do usuário (CAP-2 / CAP-3 da spec)
log("Mnemo: indexando knowledge/")
index("knowledge/")

local pergunta = (arg and arg[1] and arg[1] ~= "") and arg[1] or "o que e RAG?"
log("Perguntando: " .. pergunta)

local hits = ask(pergunta)
if #hits == 0 then
  print("Não foi encontrado.")
else
  for i, hit in ipairs(hits) do
    print(string.format("--- #%d score=%.4f source=%s ---", i, hit.score, hit.source))
    print(hit.text)
  end
end
