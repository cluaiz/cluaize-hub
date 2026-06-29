# Tutorial: Using the cluaize-db Plugin

This tutorial guides you through using the `cluaize-db` extension to manage persistent local data.

## 1. Installation
Install the database extension using the Cluaize CLI:
```bash
cluaize extension install cluaize-db
```
Verify installation:
```bash
cluaize extension list
```

## 2. Triggering via AI
The database plugin is loaded lazily. Ask the AI to store or retrieve data:
```bash
cluaize chat "Save my API key for GitHub as 'gh_token_123'"
```
The AI will automatically route this request using the CEL command:
`use extension::cluaize-db -> execute(...)`

## 3. Direct CEL Execution
You can manually execute database commands via CEL without the AI router:
```bash
cluaize run "use extension::cluaize-db -> execute(action: 'set', key: 'github_token', value: 'gh_token_123')"
```
