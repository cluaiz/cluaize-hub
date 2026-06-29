---
title: "Prompt Engineering with CEL"
description: "How to teach the LLM the exact CEL grammar required to invoke dependencies."
category: "Tutorials"
---

# 2. Prompt Engineering with CEL

When writing a Skill, you are writing instructions for a Large Language Model (LLM). Unlike standard compiled code, LLMs are probabilistic. If you tell an LLM to "use the database," it might try to hallucinate a SQL query or write python code.

To prevent hallucinations, you must use **Explicit CEL Grammar** in your `SKILL.md`.

---

## Teaching the AI its Tools

In Cluaize, the AI interacts with the system using **CEL (Cluaize Execution Language)**. You must provide the exact CEL template in your prompt so the AI knows how to construct the command.

### Bad Example (Vague)
```markdown
# Bad Prompt
If the user asks for their balance, fetch it from the database extension.
```
*Result:* The AI will hallucinate `SELECT balance FROM users;` which the Engine cannot parse.

### Good Example (Explicit Grammar)
```markdown
# Good Prompt
If the user asks for their balance, fetch it from the database extension using the following CEL command:

`use extension::cluaize-db-engine -> execute(query: "get_balance", user_id: "<DYNAMIC_ID>")`

Replace `<DYNAMIC_ID>` with the actual user ID. Wait for the engine to return the result before answering.
```
*Result:* The AI outputs perfect CEL. The Engine's Router intercepts it, compiles the payload, invokes the C-Pointer for the database extension, and returns the balance directly into the AI's context.

## Handling Multiple Tools

One of the most powerful features of Skills is chaining. You can teach the AI to chain an MCP Server and a WASM Plugin together.

```markdown
# Data Processing Pipeline

To process incoming text, follow these exact steps:

1. Fetch the raw text from the GitHub MCP:
   `let $raw_text = use mcp::github-mcp-connector -> call_tool(read_issue, id: "123");`
   
2. Pass the result to the math WASM plugin to extract statistics:
   `let $stats = use plugin::cluaize-math-accelerator -> invoke(calculate, expr: $raw_text);`
   
3. Summarize the `$stats` output for the user.
```

By providing these precise templates, the AI acts as an orchestrator, seamlessly routing variables between independent muscle components.
