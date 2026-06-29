# Skill format

## On disk

A skill is a folder inside `skills/<category>/`.

Required:

- `SKILL.md` (or `skill.md`)

Optional:

- Any supporting files: `logic.wasm`, `state.kvcache.bin`, config JSONs, scripts,
  reference docs, templates.

Installed skill metadata (written by the CLI):

- `<skill>/.cluaiz/origin.json`

## `SKILL.md`

Markdown with YAML frontmatter. The frontmatter declares metadata. The Markdown
body is the agent's system prompt — it tells the agent what the skill
does, when to activate it, how to use its tools, and what rules to follow.

## Frontmatter metadata

### Basic frontmatter

```yaml
---
name: my-skill
description: Short summary of what this skill does and when to trigger it.
version: 1.0.0
---
```

### Full frontmatter (Cluaiz native skill)

```yaml
---
id: cluaiz.skill.ops.storage
name: storage-probe
version: 1.0.0
description: Hardware-level storage health and speed benchmarking.
author: Cluaiz
soul_type: STEERING_VECTOR

compatibility:
  min_hidden_dim: 2048
  model_families:
    - UNIVERSAL

permissions:
  filesystem: true
  network: false
  level: ReadOnly
  mcp_servers: []

triggers:
  semantic:
    - storage-probe
    - disk-benchmark
  entropy_threshold: 0.7

links:
  wasm: "./logic.wasm"
  prompt_cache: "./state.kvcache.bin"
---
```

### Field reference

| Field | Type | Description |
|---|---|---|
| `id` | `string` | Unique skill identifier (e.g., `cluaiz.skill.ops.storage`). |
| `name` | `string` | Skill name. Must match the directory name. Lowercase `kebab-case`. |
| `version` | `string` | Semver version string. |
| `description` | `string` | What this skill does. Also used as the summary in registry search. |
| `author` | `string` | Skill author or organization. |
| `soul_type` | `string` | Engine execution mode: `PROMPT_CACHE`, `STEERING_VECTOR`, `LORA_PATCH`, or `markdown`. |
| `compatibility.min_hidden_dim` | `int` | Minimum model hidden dimension required. |
| `compatibility.model_families` | `string[]` | Model families supported (e.g., `UNIVERSAL`, `LLAMA`, `MISTRAL`). |
| `permissions.filesystem` | `bool` | Whether the skill can read/write the filesystem. |
| `permissions.network` | `bool` | Whether the skill can make network requests. |
| `permissions.level` | `string` | Access level: `ReadOnly`, `ReadWrite`, or `Admin`. |
| `permissions.mcp_servers` | `string[]` | Allowed MCP server connections. |
| `triggers.semantic` | `string[]` | Semantic keywords that activate this skill. |
| `triggers.entropy_threshold` | `float` | Confidence threshold (0.0–1.0) before the agent loads this skill. |
| `links` | `object` | Paths to associated asset files (see "Linking assets" below). |

### Linking assets

The `links` object in the frontmatter maps asset types to relative file paths.
The Cluaiz engine reads these links at runtime and loads the referenced files
alongside the skill.

```yaml
links:
  wasm: "./logic.wasm"
  prompt_cache: "./state.kvcache.bin"
  config: "./settings.json"
  mcp: "./connector.json"
```

| Link key | File type | Purpose |
|---|---|---|
| `wasm` | `.wasm` | WebAssembly binary. High-performance native logic executed in a secure sandbox. Zero-copy, memory-safe. |
| `prompt_cache` | `.kvcache.bin` | Persistent key-value memory. Stores historical data, baselines, or session state across agent runs. |
| `config` | `.json` / `.yaml` | Configuration data the agent reads at runtime (API endpoints, default parameters, tool schemas). |
| `mcp` | `.json` / `.js` | MCP (Model Context Protocol) server connector. Allows the agent to communicate with local or remote tool servers. |
| `db` | `.db` / `.sqlite` | Embedded database for skills that need structured query access. |
| `scripts` | `./scripts/` | Directory of helper scripts (Python, Bash) the agent can invoke. |

All paths are relative to the skill folder.

## The agent prompt (Markdown body)

Everything below the closing `---` of the frontmatter is the agent prompt. This
is loaded directly into the agent's context window.

**This is not a user-facing README.** Write it as direct instructions to the AI.

A well-structured agent prompt includes:

### 1. Skill identity

Tell the agent what capability it now has.

```markdown
# Skill: Storage Probe

You are equipped with the **Storage Probe** native skill. This skill allows you
to run low-level hardware diagnostics on NVMe and SSD drives.
```

### 2. When to use this skill

Explicit trigger conditions so the agent knows when to activate.

```markdown
## When to use this skill
Trigger this skill when the user asks you to:
1. Benchmark read/write speeds of a disk.
2. Check S.M.A.R.T. health data.
3. Diagnose slow IO or system bottlenecks.
```

### 3. How to use this skill (Associated files)

Explain what each linked file does and how the agent should interact with it.

```markdown
## How to use this skill (Associated Files)

- **`logic.wasm`**: The core execution binary. Invoke via `cluaiz-run`.
- **`state.kvcache.bin`**: Historical baseline metrics. Read before giving results.
```

### 4. Execution instructions

Concrete commands or API calls the agent should run.

```markdown
### Execution Instructions
cluaiz-run ./logic.wasm --mode <read|write|health> --target <path>
```

### 5. Guardrails and rules

Hard constraints the agent must obey.

```markdown
## Guardrails and Rules
1. NEVER run write benchmarks on OS directories without explicit consent.
2. Always compare results against kvcache.bin baselines.
3. No network access. Do not send results externally.
```

## Allowed files

Only text-based files and the following binary types are accepted:

- `.wasm` — WebAssembly binaries
- `.kvcache.bin` — Key-value cache state files
- `.db` / `.sqlite` — Embedded databases

Text file types: `.md`, `.json`, `.yaml`, `.yml`, `.toml`, `.js`, `.ts`, `.py`,
`.sh`, `.txt`, `.csv`, `.svg`.

## Naming conventions

- Skill directory names: lowercase `kebab-case` (e.g., `pdf-extractor`, `code-auditor`).
- Category directory names: lowercase `kebab-case` (e.g., `dev-suite`, `system-ops`).
- `SKILL.md` filename: uppercase `SKILL.md` preferred, lowercase `skill.md` accepted.

## Versioning

Each skill declares its version in the frontmatter. The registry tracks the
latest published version. When a skill is updated, bump the version following
semver conventions.
