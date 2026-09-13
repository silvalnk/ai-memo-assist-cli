-- MemoAssist — user CLI (CAP-2 / CAP-3)
log("MemoAssist: indexing knowledge/")
index("knowledge/")

local question = (arg and arg[1] and arg[1] ~= "") and arg[1] or "o que e RAG?"
log("Asking: " .. question)

local hits = ask(question)
if #hits == 0 then
  print("Not found.")
else
  for i, hit in ipairs(hits) do
    print(string.format("--- #%d score=%.4f source=%s ---", i, hit.score, hit.source))
    print(hit.text)
  end
end
