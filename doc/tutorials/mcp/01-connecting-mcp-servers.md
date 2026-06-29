---
title: "Connecting MCP Servers"
description: "How to integrate external Model Context Protocol (MCP) servers with the Cluaize Engine via stdio."
category: "Tutorials"
---

# 1. Connect Model Context Protocol (MCP) Servers

The **Model Context Protocol (MCP)** is a standardized way for AI models to securely connect to external data sources and tools (e.g., GitHub, Slack, local file systems).

Unlike Extensions (Native DLLs) or Plugins (WASM) which run within the Cluaize Engine memory space, MCP servers are **separate processes** (e.g., Node.js or Python scripts).

> [!TIP]
> **For more details, reference:** [`skill_architecture.md`](file:///c:/Users/Aryan/my/Cluaiz-workspace/Cluaiz-Technologies/cluaize-hub/doc/architecture/skill_architecture.md)
> 
> For **MCP Servers**, creating a `SKILL.md` file is **OPTIONAL but RECOMMENDED**. Providing a `SKILL.md` teaches the AI exactly how to structure the JSON-RPC arguments for the MCP tool and gives the AI context on why it should call the tool.

---

## The MCP Architecture

Because MCP servers run externally, the Engine spawns them as background processes and communicates via standard input/output (`stdio`) streams using JSON-RPC.

```mermaid
flowchart LR
    A["Cluaize Engine"] -->|stdio JSON-RPC| B["MCP Process Manager"]
    B -->|Spawns (npx / python)| C["MCP Server Process"]
    C -->|Reads External APIs| D["GitHub / Slack"]
    D -->|Returns JSON-RPC| B
    B -->|Context Injection| A
```

## Step 1: Defining the MCP Manifest

To register an external MCP server with Cluaize, create a `manifest-mcp.yaml`. Notice how the `execution` block uses `command` and `args` instead of `entry_point`.

```yaml
# =====================================================================
# CLUAIZE MCP MANIFEST (manifest-mcp.yaml)
# =====================================================================
name: "github-mcp-connector"
version: "1.0.0"
description: "Connects the engine to GitHub via official MCP server."
author: "Cluaiz Community"
type: "mcp"

discovery:
  semantic_triggers: ["github", "pull request", "issues", "repo"]
  cel_grammar: "use mcp::github-mcp-connector -> call_tool(...)"

activation:
  # Don't run the Node.js process until the AI actually asks for it.
  lazy_load: true
  trigger_on: 
    - "on_command:use mcp::github-mcp-connector"

permissions:
  # MCP processes manage their own memory, so Engine doesn't cap it.
  max_memory_mb: null
  # Long timeout because network APIs take time.
  max_cpu_time_ms: 30000
  network_access: true
  allowed_hosts: ["api.github.com"]
  mid_layer_jit_injection: false
  file_system: "none"

execution:
  # The terminal command to run.
  command: "npx"
  # Arguments passed to the command.
  args: ["-y", "@modelcontextprotocol/server-github"]
  # Environment variables injected into the OS process.
  env:
    GITHUB_PERSONAL_ACCESS_TOKEN: "${GITHUB_TOKEN}"
```

> [!WARNING]
> Ensure that you do not expose sensitive API keys in plaintext in the `manifest-mcp.yaml` file. Always use the `${ENV_VAR}` syntax to resolve secrets from the host system environment dynamically.
