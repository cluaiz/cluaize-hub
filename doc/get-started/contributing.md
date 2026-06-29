# Contributing to Cluaiz Skills

Thank you for contributing to the Cluaiz skill ecosystem.

## Pull Request Requirements

### Title format

Use [Conventional Commits](https://www.conventionalcommits.org/) style:

```
feat(<skill-name>): add new skill for X
fix(<skill-name>): fix frontmatter parsing
docs: update skill-format reference
chore: update CI workflow
```

### Scope

**One PR, one purpose.** Each PR should do exactly one of:

- Add a new skill or plugin
- Fix a bug in an existing skill or plugin
- Improve documentation

Do not bundle unrelated changes together.

### PR description

Every PR must include:

1. **What** — what you added or changed
2. **Why** — the motivation or use case

## Skill Structure

### Directory layout

```
skills/<category>/<skill-name>/
├── SKILL.md                 # Required — entry point with YAML frontmatter
├── logic.wasm               # Optional — native execution binary
├── state.kvcache.bin           # Optional — persistent memory
├── scripts/                 # Optional — helper scripts
│   └── *.py
└── references/              # Optional — detailed reference docs
    └── *.md
```

- The directory name is the skill identifier. Use lowercase `kebab-case`
  (e.g., `pdf-extractor`).
- `SKILL.md` is the only required file.

### SKILL.md frontmatter

```yaml
---
name: my-skill                    # Required — must match directory name
description: >                    # Required — what this skill does and when to
  Short description including       trigger it
  trigger conditions.
version: 1.0.0                    # Required
author: Your Name                 # Recommended
---
```

**Required fields:** `name`, `description`, `version`

- `name` must exactly match the directory name.
- `description` must clearly state trigger conditions — this is what the agent
  uses to decide whether to load your skill.

See [Skill format](./skill-format.md) for the full frontmatter reference.

### No hardcoded secrets

**Never hardcode API keys, tokens, or credentials in any file.**

If your skill calls an external API, instruct the agent to read credentials
from environment variables:

```python
API_KEY = os.getenv("MY_API_KEY")
if not API_KEY:
    raise SystemExit("ERROR: MY_API_KEY is not set.")
```

Document required environment variables in your `SKILL.md`.

## Guidelines

### 1. Avoid overlap

Before creating a new skill, check existing skills for functional overlap. If
your feature could extend an existing skill, prefer extending over creating new.

### 2. Keep files focused

Skills are loaded into the agent's context window. Every token counts.

- Keep `.md` files concise and focused.
- Split large reference docs into parts.
- Do not embed large data blobs in Markdown files.

### 3. Script standards

If your skill includes helper scripts:

- Include a shebang line (e.g., `#!/usr/bin/env python3`).
- Handle errors gracefully with clear messages.
- Document script usage in the `SKILL.md`.

### 4. Naming and encoding

- Skill names and file names: ASCII only, `kebab-case`.
- All content in English.
- All files UTF-8 encoded.

## Review process

1. Submit your PR following the requirements above.
2. At least one maintainer will review.
3. Address review feedback.
4. Once approved, a maintainer will merge.

## Questions?

Open an issue if you have questions about contributing.
