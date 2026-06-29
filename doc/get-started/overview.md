# Cluaiz Skills & Plugins

Cluaiz Skills and Plugins are the extension layer for the Cluaiz Independent OS.
They teach the Independent Agent new capabilities — from running storage diagnostics
to generating PDFs to managing GitHub repositories — all while enforcing strict
zero-trust security at the hardware level.

## What is a Skill?

A skill is a folder containing a `SKILL.md` file and optional supporting assets.

The `SKILL.md` has two parts:

1. **YAML frontmatter** — machine-readable metadata (name, permissions, triggers,
   linked assets).
2. **Markdown body** — the agent prompt. This is a direct system instruction that
   tells the Independent Agent *when* to use the skill, *how* to invoke its tools,
   and *what rules* to follow.

The Markdown body is **not** a user-facing README. It is loaded directly into the
agent's context window at runtime. Every word matters — write it as if you are
briefing a highly capable operator.

## What is a Plugin?

A plugin is a folder containing a `SKILL.md` and external tool connectors (MCP
servers, API bridges, search engine integrations). Plugins extend the agent's
ability to interact with the outside world through standardized protocols.

## Skills vs Plugins vs Souls

|              | Skills                                         | Plugins                                          | Souls                                                            |
| ------------ | ---------------------------------------------- | ------------------------------------------------ | ---------------------------------------------------------------- |
| **Purpose**  | Teach the agent a new capability               | Connect the agent to an external service or tool | Define the agent's core identity, persona, and overarching rules |
| **Location** | `skills/<category>/<name>/`                    | `plugins/<category>/<name>/`                     | `souls/<name>/`                                                  |
| **Assets**   | `logic.wasm`, `state.kvcache.bin`, config files   | `connector.json`, `connector.js`, MCP configs    | None (Pure `SOUL.md` identity)                                   |
| **Network**  | Typically offline (`network: false`)           | Typically online (`network: true`)               | Offline                                                          |
| **Example**  | PDF extraction, code auditing, VRAM management | Brave Search, GitHub API, database connectors    | Independent Hacker, Data Scientist, Marketing Copywriter           |

## How it works

1. The Cluaiz CLI reads the central `registry.json` to discover available skills
   and plugins.
2. When you run `cluaiz skill install <name>`, the CLI downloads the skill folder
   (including `SKILL.md` and all linked assets) to `~/.cluaiz/skills/<name>/`.
3. At runtime, the Independent Agent reads the `SKILL.md` to understand what the
   skill does and how to use it. If the YAML frontmatter links a `logic.wasm`
   binary, the agent can invoke it for high-performance native execution.
4. Permissions declared in the frontmatter (`filesystem`, `network`, `level`) are
   enforced by the Cluaiz sandbox. The agent cannot exceed what the skill declares.

## Repository structure

```
cluaiz-skills/
├── skills/                      # All skills, organized by category
│   ├── productivity/
│   │   ├── pdf-extractor/
│   │   │   ├── SKILL.md         # Required — the skill definition
│   │   │   ├── logic.wasm       # Optional — native execution binary
│   │   │   └── state.kvcache.bin   # Optional — persistent memory
│   │   └── calendar-scheduler/
│   │       └── SKILL.md
│   ├── dev-suite/
│   │   └── ...
│   └── Independent-ops/
│       └── ...
│
├── plugins/                     # All plugins, organized by category
│   ├── search-engines/
│   │   └── brave-search/
│   │       ├── SKILL.md
│   │       └── connector.json
│   └── databases/
│       └── ...
│
├── souls/                       # Core personas and identities
│   ├── Independent-hacker/
│   │   └── SOUL.md
│   └── ...
│
├── doc/                         # Documentation (this folder)
├── scripts/                     # Registry build scripts
├── .github/                     # CI/CD workflows
├── README.md
├── LICENSE
└── registry.json                # Auto-generated skill/plugin index
```

## Quick start

Install a skill:

```bash
cluaiz skill install <name>
```

Search for skills:

```bash
cluaiz skill search "pdf"
```

Update all installed skills:

```bash
cluaiz skill update --all
```

See [Quickstart](./quickstart.md) for a full walkthrough.
