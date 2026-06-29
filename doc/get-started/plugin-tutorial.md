# Tutorial: Using Plugins and Skills

This step-by-step guide will teach you how to install, configure, and use a plugin (or skill) in the Cluaize Engine. 

By the end of this tutorial, you will have successfully loaded the `cluaize-search` plugin and used it to fetch live web data.

## Prerequisites
- Cluaize Engine installed and running.
- Cluaize CLI installed (`cluaize` command available in your terminal).

---

## Step 1: Install the Plugin
First, we need to download the plugin into the engine's registry. Use the CLI to install the `cluaize-search` extension.

```bash
cluaize extension install cluaize-search
```
The engine will download the plugin and automatically place it in its required `storage_domain` (e.g., `~/.cluaize/extensions/cluaize-search`). It also registers it in the master `registry.yaml` as a Lazy-loaded component.

## Step 2: Verify Installation
Verify that the plugin is recognized by the engine:

```bash
cluaize extension list
```
You should see `cluaize-search` listed with its status as `enabled: true`.

## Step 3: Trigger the Plugin via AI
Because Cluaize uses an EventBus for Lazy-loaded components, you don't need to manually start any background processes. You simply ask the AI a question that matches the plugin's semantic keywords (like "search" or "web").

Open the Cluaize TUI dashboard or use the CLI:
```bash
cluaize chat "What is the latest stable version of Rust?"
```

Behind the scenes, the AI engine will:
1. Detect that your prompt requires web access.
2. Emit the CEL (Cluaize Expression Language) command defined in the plugin's manifest: `use extension::cluaize-search -> query(q: 'latest stable version of Rust')`.
3. The engine parses this CEL command, instantly loads the plugin's binary into RAM, executes the native FFI call, and injects the results back into your chat context window.

## Step 4: Direct CEL Execution (Advanced)
If you are writing scripts or testing the engine without the AI router, you can execute the CEL command directly using the CLI:

```bash
cluaize run "use extension::cluaize-search -> query(q: 'latest stable version of Rust')"
```

## Summary
You have successfully:
1. Installed a plugin via the CLI.
2. Verified its registration in the engine.
3. Triggered it using natural language.
4. Triggered it using direct CEL syntax.
