---
title: "CEL vs MCP Execution"
description: "How the Cluaize Engine routes CEL scripts into standard MCP JSON-RPC streams."
category: "Tutorials"
---

# 3. CEL Architecture for MCP Servers

Like Plugins and Extensions, MCP Servers **do not parse CEL.** They only speak standard JSON-RPC over `stdio`. The Engine acts as the universal translator between the AI's CEL script and the MCP protocol.

## How The AI Evaluates an MCP Tool

When the AI model determines it needs data from GitHub, it generates a CEL script based on the `cel_grammar` defined in the manifest.

```cel
let $prs = use mcp::github-mcp-connector -> call_tool(list_pull_requests, owner: "cluaiz", repo: "engine");
```

### The Translation Pipeline

1. **The Engine (CEL Router)** parses the script and identifies the target: `github-mcp-connector`.
2. The Engine translates the CEL arguments into a standard MCP JSON-RPC payload:
   ```json
   {
     "jsonrpc": "2.0",
     "id": 1,
     "method": "tools/call",
     "params": {
       "name": "list_pull_requests",
       "arguments": {
         "owner": "cluaiz",
         "repo": "engine"
       }
     }
   }
   ```
3. The Engine writes this JSON string to the MCP process's **`stdin`**.
4. The MCP Server executes the network request to GitHub.
5. The MCP Server writes the JSON result to its **`stdout`**.
6. The Engine reads `stdout`, converts it back into the CEL variable `$prs`, and resumes the AI inference.

## Why use CEL for MCPs?

By using CEL as the intermediary, the AI Model doesn't need to learn the complex JSON-RPC specification or worry about request IDs. 

It simply writes a one-line function call (`use mcp::... -> call_tool(...)`), and the Engine perfectly formats and routes the request to the underlying Node.js or Python process.
