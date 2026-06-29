---
title: "Fetching Settings for MCP"
description: "How the Engine injects system bindings into the MCP process environment."
category: "Tutorials"
---

# 2. Fetching Settings & Variables in MCP

Unlike Native Extensions or WASM Plugins—which receive a C-Pointer containing a MsgPack or Bincode payload—MCP servers are entirely separate processes (e.g., Node.js). 

They cannot read C-Pointers. So, how does an MCP server read Cluaize Engine settings like `system_booster.think_mode`?

## The Solution: Process Environment Variables

When the Cluaize Engine spawns the MCP process via `stdio`, it resolves the requested `system_bindings` and passes them as **Environment Variables** to the child process.

---

## Step 1: Subscribe to System Bindings

In your `manifest-mcp.yaml`, request the variables you need:

```yaml
# ---------------------------------------------------------------------
# SYSTEM BINDINGS (Dynamic Hardware & Booster State)
# ---------------------------------------------------------------------
system_bindings:
  - "system_booster.think_mode"
  - "system_booster.response_length"
```

## Step 2: Reading the Variables in the MCP Server

Because the Engine injects these bindings directly into the OS environment of the spawned process, the MCP server simply reads them like any other environment variable.

The Engine converts dot-notation (`system_booster.think_mode`) into double-underscore screaming snake case (`SYSTEM_BOOSTER__THINK_MODE`).

### Node.js Example (TypeScript/JavaScript)
```typescript
import { McpServer } from "@modelcontextprotocol/sdk/server/mcp.js";

const server = new McpServer({
  name: "custom-github-connector",
  version: "1.0.0"
});

// Read the Cluaize injected settings
const thinkMode = process.env.SYSTEM_BOOSTER__THINK_MODE || "Off";
const responseLength = process.env.SYSTEM_BOOSTER__RESPONSE_LENGTH || "auto";

console.error(`Starting MCP with Think Mode: ${thinkMode}`);

server.tool(
  "fetch_repo_data",
  async ({ repoUrl }) => {
      // Adjust the depth of API calls based on Cluaize's think_mode!
      if (thinkMode === "On") {
          return { content: [{ type: "text", text: "Deep fetching all issues and PRs..." }] };
      } else {
          return { content: [{ type: "text", text: "Shallow fetch complete." }] };
      }
  }
);
```

### Python Example
```python
import os
from mcp.server import Server

app = Server("custom-github-connector")

# Read the Cluaize injected settings
think_mode = os.environ.get("SYSTEM_BOOSTER__THINK_MODE", "Off")

@app.tool()
async def fetch_repo_data(repo_url: str) -> str:
    if think_mode == "On":
        return "Deep fetching all issues and PRs..."
    return "Shallow fetch complete."
```

> [!TIP]
> This design allows you to use standard, off-the-shelf MCP servers and still pass Engine-specific context to them without modifying their source code, simply by mapping environment variables in the execution block!
