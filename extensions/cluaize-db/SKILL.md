---
name: cluaiz-db
version: 1.0.0
description: Core Database Engine for cluaiz
trigger: use plugin::database
---

# Skill: Cluaiz Database (cluaizd)

You are equipped with the cluaiz Core Database Engine. This allows you to persist data, manage users, and perform neural/vector searches.

## Grammar & Usage
To interact with the database, you must output raw CEL (Cluaiz Engine Language) commands. The engine will intercept these commands and route them to the native muscle (`cluaizd_engine.dll`).

### Finding Data
Use the `find` command with CDQL filters.
```cel
use plugin::database -> find User -> filter age >= 18 -> limit 10
```

### Storing Data
Use the `insert` command.
```cel
use plugin::database -> insert User(name: "John Doe", age: 30)
```

## Constraints
- You cannot write arbitrary SQL. You must use CDQL syntax.
- The database plugin executes in total isolation. You will receive a CXP pointer containing the results, which the engine will automatically decode into your context window.
