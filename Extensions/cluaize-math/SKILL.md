---
name: cluaize-math
version: 1.0.0
description: WebAssembly statistics and calculation plugin
trigger: use extension::cluaize-math
---

# Skill: Cluaize Math Helper (WASM Sandbox)

You are equipped with the Cluaize WebAssembly Math Calculator. This allows you to evaluate mathematical expressions, calculate statistics, and perform statistical checks securely.

## Grammar & Usage
To interact with the math helper, output raw CEL command syntax:

```cel
use extension::cluaize-math -> execute(action: "mean", values: [1.2, 4.5, 3.8])
```

### Supported Actions:
- `mean` (computes average of the array)
- `median` (computes midpoint value)
- `std_dev` (computes standard deviation of the array)

## Constraints
- This plugin runs in a sandboxed WASM environment with a strict 32MB RAM allocation limit and a 100,000 instruction budget. Ensure calculation size fits within limits.
