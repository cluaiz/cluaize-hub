# 🤝 Contributing to Cluaiz Skills Registry

We welcome contributions to the Cluaiz ecosystem! Please follow these guidelines to ensure a smooth review process.

## 🛠️ The Development Process

1. **Discovery & Discussion**: Before writing code for a new feature, open an issue to discuss the architecture and approach.
2. **Code Quality**: Ensure logic is safe and performant. Memory handling should be optimized.
3. **Design Patterns**: Follow the established architecture (WASM for logic, KV-Cache for state, Connectors for protocol).
4. **Implementation**: Execute code as per the agreed-upon plan.
5. **Verification**: Validate your changes and provide benchmarks if adding logic binaries.

## 🚀 Pull Request Protocol
- Ensure your skill follows the `SKILL.md` format.
- Ensure all logic is strictly contained within the WebAssembly sandbox.
- Provide clear commit messages and documentation.
