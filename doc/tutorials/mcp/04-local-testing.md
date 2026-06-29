---
title: "Testing MCP Servers Locally"
description: "How to test your MCP server directly in the terminal."
category: "Tutorials"
---

# 4. Local Testing for MCP Servers

Testing an MCP server is incredibly simple compared to Extensions or Plugins. Because MCP servers are standard command-line applications that read from `stdin` and write to `stdout`, you don't even need a Rust unit test or a Named Pipe to test them.

---

## Step 1: Export Environment Variables

Since the Engine usually handles injecting `system_bindings` and `env` blocks into the process, you must simulate this manually in your terminal before running the server.

### Windows (PowerShell)
```powershell
$env:GITHUB_PERSONAL_ACCESS_TOKEN = "your_actual_token_here"
$env:SYSTEM_BOOSTER__THINK_MODE = "On"
```

### Linux / macOS (Bash)
```bash
export GITHUB_PERSONAL_ACCESS_TOKEN="your_actual_token_here"
export SYSTEM_BOOSTER__THINK_MODE="On"
```

## Step 2: Run the Server Manually

Run the exact command specified in your `manifest-mcp.yaml` `execution` block.

```bash
npx -y @modelcontextprotocol/server-github
```

If the server boots correctly, it will appear to hang. This is expected! It is waiting for you to type a JSON-RPC request into `stdin`.

## Step 3: Send a Mock Request

Paste the following standard MCP JSON-RPC payload into your terminal and press Enter:

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

If your MCP server is functioning properly, it will instantly print a JSON-RPC response back to the terminal (`stdout`) containing the GitHub data. 

If this manual terminal test works, you can guarantee that the Cluaize Engine will be able to spawn and communicate with it!
