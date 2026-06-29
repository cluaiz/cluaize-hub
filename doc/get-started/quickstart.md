# Quickstart

Create, test, and publish a Cluaiz skill in 5 minutes.

## 1. Create the skill folder

Pick a category and a name. Use lowercase `kebab-case` for both.

```bash
mkdir -p skills/productivity/my-skill
```

## 2. Write the SKILL.md

Create `skills/productivity/my-skill/SKILL.md`:

```markdown
---
name: my-skill
version: 1.0.0
description: >
  Summarize long documents into concise bullet points. Use when the user asks
  to summarize, condense, or extract key points from text or files.
author: Your Name

permissions:
  filesystem: true
  network: false
  level: ReadOnly

triggers:
  semantic:
    - summarize
    - bullet-points
    - key-points
  entropy_threshold: 0.6
---

# Skill: Document Summarizer

You are equipped with the **Document Summarizer** skill. This skill allows you
to condense long-form text into clear, structured bullet points.

## When to use this skill
Trigger this skill when the user asks you to:
1. Summarize a document, article, or file.
2. Extract key points or takeaways.
3. Condense meeting notes or transcripts.

## How to use this skill
1. Read the full input text provided by the user.
2. Identify the 5-10 most important points.
3. Present them as concise bullet points, ordered by importance.

## Guardrails and Rules
1. Never fabricate information not present in the source text.
2. Preserve critical numbers, dates, and proper nouns exactly.
3. If the input is shorter than 100 words, tell the user it does not need
   summarization.
```

That's it. One file. The frontmatter is the metadata, the body is the prompt.

## 3. Add supporting files (optional)

If your skill needs native execution or persistent memory, add asset files
alongside the `SKILL.md` and link them in the frontmatter:

```
skills/productivity/my-skill/
├── SKILL.md
├── logic.wasm           # Optional — native execution binary
├── state.kvcache.bin       # Optional — persistent memory
└── config.json          # Optional — runtime configuration
```

Link them in the YAML:

```yaml
links:
  wasm: "./logic.wasm"
  prompt_cache: "./state.kvcache.bin"
  config: "./config.json"
```

Then explain each file in the Markdown body so the agent knows how to use them.

## 4. Test locally

Install your skill from the local registry:

```bash
cluaiz skill install my-skill
```

The CLI will download the `SKILL.md` and all linked assets to
`~/.cluaiz/skills/my-skill/`.

## 5. Publish

Commit your skill folder and push to the `cluaiz-skills` repository. The
GitHub Action will automatically rebuild `registry.json` and create a release.

```bash
git add skills/productivity/my-skill/
git commit -m "feat(my-skill): add document summarizer skill"
git push
```

## Next steps

- Read [Skill format](./skill-format.md) for the complete frontmatter reference.
- Read [Plugin format](./plugin-format.md) if your skill connects to external services.
- Read [Contributing](./contributing.md) for PR requirements and review process.
