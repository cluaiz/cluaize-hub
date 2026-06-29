---
title: "Creating SKILL.md for Tools"
description: "How to properly build a SKILL.md file to give AI the power to understand and use Extensions, Plugins, and MCPs."
category: "Tutorials"
---

# 4. Creating `SKILL.md` for Your Tools

> [!TIP]
> **For full architectural details, reference:** [`skill_architecture.md`](file:///c:/Users/Aryan/my/Cluaiz-workspace/Cluaiz-Technologies/cluaize-hub/doc/architecture/skill_architecture.md)

When you build a new tool for Cluaize (whether it's an Extension, a WASM Plugin, or an MCP Server), you are building **Muscle**. 

However, the AI does not automatically know how to use your tool. To give the AI power, context, and exact instructions, you must create a `SKILL.md` file.

### Requirement Levels
- **Extensions:** Strict Requirement. You must write a `SKILL.md` to safely orchestrate raw OS access.
- **Plugins & MCPs:** Optional, but highly recommended. Without it, the AI might guess the tool's usage and hallucinate parameters.

---

## The `SKILL.md` Structure

A `SKILL.md` file consists of two parts:
1. **YAML Frontmatter:** For Engine routing and limits.
2. **Markdown Body:** The system prompt injected into the AI.

### Example `SKILL.md` for a Plugin

```markdown
---
title: "Fast Math Accelerator"
version: "1.0.0"
description: "Equips the AI with an exact, hardware-accelerated math calculation tool."
author: "Cluaiz"
soul_type: "PROMPT_CACHE"
keywords: ["math", "calculator", "add", "multiply", "divide"]
triggers:
  semantic: ["calculate", "math problem"]
  entropy_threshold: 0.82
permissions:
  level: "ReadOnly"
  filesystem: false
  network: false
core_metadata:
  token_count: 145
---

# Math Execution Protocol

You are equipped with the `cluaize-math-accelerator` plugin. When the user asks you to perform a calculation, you MUST NOT calculate it yourself (to avoid hallucination).

Instead, you must use the plugin by emitting the following CEL command:

`let $result = use plugin::cluaize-math-accelerator -> invoke(calculate, expr: "<THE_EXPRESSION>");`

Once the engine returns `$result`, you must present the final number clearly to the user.
```

---

## Understanding the Frontmatter

The Engine uses the YAML frontmatter to index the skill and route requests without blowing up VRAM.

| Field | Purpose |
|-------|---------|
| `description` | Dense summary (**Max 360 chars**). The Engine feeds this directly into the embedding model to represent the skill in vector space. |
| `keywords` | Rapid matching triggers (**Max 10 tags**). Exceeding 10 causes "Semantic Entropy Dilution," degrading the engine's confidence score. |
| `triggers.entropy_threshold` | Strict confidence baseline (0.0 to 1.0). If the semantic match score is below this, the engine aborts loading to prevent hallucinations. |
| `core_metadata.token_count` | The exact token footprint of your markdown body. The VRAM Arbiter uses this to verify GPU memory before injecting the prompt, preventing OOM crashes. |

---

## Why Provide a `SKILL.md`?

1. **Eliminate Hallucination:** If you just expose a "database" MCP, the AI might hallucinate SQL. By providing a `SKILL.md`, you teach it the *exact* CEL syntax.
2. **VRAM Efficiency:** Instead of dumping instructions in a global prompt, the `SKILL.md` is lazily loaded only when the `entropy_threshold` is met.
3. **Safety Boundaries:** The frontmatter defines explicit `permissions.level` (e.g., `ReadOnly`), ensuring the AI cannot accidentally trick your tool into deleting files.
