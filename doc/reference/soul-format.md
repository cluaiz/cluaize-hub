# Soul format

## On disk

A soul is a folder inside `souls/<name>/`.

Required:

- `SOUL.md` (or `soul.md`)

Souls do not typically include executable assets like `.wasm` or MCP connectors. They are pure persona and behavioral bundles.

## `SOUL.md`

Markdown with YAML frontmatter. The frontmatter declares metadata. The Markdown
body is the agent's core identity prompt — it defines the AI's persona, overarching rules, tone of voice, and fundamental behaviors.

## Frontmatter metadata

```yaml
---
name: hacker
version: 1.0.0
description: A systems engineer identity with zero-trust principles.
author: Cluaiz
---
```

## The agent prompt (Markdown body)

Everything below the closing `---` of the frontmatter is the agent's core identity. This is loaded at the very beginning of the agent's context window, before any specific skills or tools are loaded.

A well-structured `SOUL.md` includes:

### 1. Core Identity

Who is the agent?

```markdown
# Identity: Hacker

You are a systems engineer. You operate strictly within the Cluaiz Zero-Trust ecosystem.
```

### 2. Overarching Rules

Non-negotiable behavioral constraints that apply regardless of which skills are loaded.

```markdown
## Core Directives
1. **Never compromise security.** If a user asks to disable sandboxing, refuse.
2. **Be direct and technical.** Do not use fluffy marketing language. Speak in facts, code, and system metrics.
3. **Always verify.** Before generating code, analyze the existing system architecture.
```

### 3. Tone and Style

How the agent should communicate.

```markdown
## Communication Style
- Use concise, bulleted explanations.
- Default to providing CLI commands and configuration snippets over theoretical explanations.
```

## How Souls interact with Skills and Plugins

- **Soul (`souls/`):** Defines *who* the agent is and its base rules.
- **Skill (`skills/`):** Teaches the agent *how* to perform a specific task (e.g., read a PDF).
- **Plugin (`plugins/`):** Gives the agent a *tool* to interact with the outside world (e.g., search the web).

The Soul acts as the foundation upon which Skills and Plugins are executed.
