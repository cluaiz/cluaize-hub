# Plugin format

## On disk

A plugin is a folder inside `plugins/<category>/`.

Required:

- `SKILL.md` (or `skill.md`) — same format as skills.

Optional:

- Connector files: `connector.json`, `connector.js`, `connector.py`
- MCP server configs
- API schema files
- Helper scripts

## How plugins differ from skills

Skills teach the agent *new behaviors* using local assets (WASM, KV-Cache).
Plugins connect the agent to *external services* through standardized protocols.

A plugin's `SKILL.md` follows the exact same format as a skill's `SKILL.md`:
YAML frontmatter for metadata, Markdown body for agent instructions. The only
difference is what the agent does with it — plugins typically involve network
calls, API authentication, or MCP server communication.

## Connector types

### MCP connectors

MCP (Model Context Protocol) connectors allow the agent to communicate with
local or remote tool servers using a standardized request/response protocol.

Link an MCP connector in the frontmatter:

```yaml
links:
  mcp: "./connector.json"
```

The `connector.json` defines the MCP server endpoint, available tools, and
authentication:

```json
{
  "server": "localhost:8080",
  "protocol": "mcp",
  "tools": [
    {
      "name": "search",
      "description": "Search the web using Brave Search API",
      "parameters": {
        "query": { "type": "string", "required": true },
        "count": { "type": "integer", "default": 10 }
      }
    }
  ],
  "auth": {
    "type": "env",
    "key": "BRAVE_API_KEY"
  }
}
```

### API connectors

For simple REST API integrations, the connector can define endpoints directly:

```json
{
  "base_url": "https://api.example.com/v1",
  "auth": {
    "type": "bearer",
    "env": "EXAMPLE_API_KEY"
  },
  "endpoints": {
    "search": { "method": "GET", "path": "/search" },
    "create": { "method": "POST", "path": "/items" }
  }
}
```

### Script connectors

Some plugins use helper scripts (Python, Node.js) as tool bridges:

```yaml
links:
  scripts: "./scripts/"
```

The agent prompt in `SKILL.md` must document how to invoke each script,
including required environment variables and expected output format.

## Plugin frontmatter

Plugin frontmatter includes the same fields as skills, plus:

```yaml
permissions:
  network: true           # Plugins typically need network access
  mcp_servers:
    - "brave-search"      # Allowed MCP server names
```

## Plugin agent prompt

The Markdown body of a plugin's `SKILL.md` must explain:

1. What external service the plugin connects to.
2. What tools or endpoints are available.
3. How to authenticate (which env vars are needed).
4. What the agent should do if authentication fails.
5. Rate limits or usage constraints.

## Categories

Plugins are organized by function:

| Category | Examples |
|---|---|
| `search-engines` | Brave Search, Google, Bing |
| `databases` | DuckDB, Relational DB, SQLite |
| `mcp-servers` | Custom MCP tool servers |
| `communication` | Slack, Discord, Email |
| `cloud` | AWS, GCP, Azure connectors |
| `dev-tools` | GitHub, GitLab, Docker |

## Environment variables

Plugins that require API keys or credentials must:

1. Declare the required env vars in the frontmatter or connector config.
2. Document setup instructions in the `SKILL.md` agent prompt.
3. Never hardcode secrets in any file.

```yaml
# In SKILL.md frontmatter
permissions:
  network: true
  required_env:
    - BRAVE_API_KEY
```

The agent prompt should instruct the agent to check for the env var and provide
a clear error message if it is missing.
