-- MemoAssist — skills CLI (CAP-5)
log("MemoAssist: indexing knowledge/")
index("knowledge/")

local function show_hits(hits)
  if #hits == 0 then
    print("Not found.")
    return
  end
  for i, hit in ipairs(hits) do
    print(string.format("--- #%d score=%.4f source=%s ---", i, hit.score, hit.source))
    print(hit.text)
  end
end

register_skill(
  "explain_rag",
  "Search the knowledge base for what RAG is and print the snippets",
  function(args)
    show_hits(ask("o que e RAG?"))
  end
)

register_skill(
  "explain_chunking",
  "Search the knowledge base for what chunking is",
  function(args)
    show_hits(ask("o que e chunking?"))
  end
)

local name = (arg and arg[1] and arg[1] ~= "") and arg[1] or "explain_rag"

local available = {}
for _, s in ipairs(list_skills()) do
  available[s.name] = true
end

if not available[name] then
  print("Unknown skill: '" .. name .. "'.")
  print("Available skills:")
  for _, s in ipairs(list_skills()) do
    print(" - " .. s.name .. ": " .. s.description)
  end
else
  log("Skill: " .. name)
  run_skill(name, {})
end
