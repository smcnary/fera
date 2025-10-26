# Contributing to Fera

Thank you for your interest in contributing to the Fera programming language!

## Development Setup

### Prerequisites
- Rust 1.70+ (for compiler bootstrap)
- LLVM 17.0+
- A C compiler (GCC or Clang)
- Git

### Building from Source

```bash
git clone https://github.com/fera-lang/fera.git
cd fera
cargo build --release
```

The `fera` binary will be available at `target/release/fera`.

### Running Tests

```bash
cargo test
```

### Project Structure

```
fera/
├── src/              # Compiler source code
│   ├── lexer/        # Tokenization
│   ├── parser/       # AST generation
│   ├── ast/          # AST definitions
│   ├── types/        # Type checking
│   ├── hir/          # High-level IR
│   ├── codegen/      # LLVM IR generation
│   └── cli/          # Command-line interface
├── stdlib/           # Standard library
│   ├── core/         # Core (always available)
│   ├── hosted/       # Hosted OS functions
│   └── embedded/     # Embedded/bare-metal
├── examples/         # Example Fera programs
├── tests/            # Integration tests
└── docs/             # Documentation

```

## How to Contribute

### Reporting Bugs

Open an issue on GitHub with:
- Fera version (`fera --version`)
- Operating system
- Minimal code to reproduce
- Expected vs. actual behavior

### Feature Requests

Check existing issues first, then open a new one with:
- Use case description
- Proposed syntax (if applicable)
- Compatibility considerations

### Pull Requests

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/my-feature`)
3. Make your changes
4. Add tests
5. Run `cargo fmt` and `cargo clippy`
6. Commit with clear messages
7. Push and open a PR

### Code Style

- Follow Rust standard style (enforced by `rustfmt`)
- Add doc comments for public APIs
- Keep functions focused and testable

### Commit Messages

Use conventional commits format:
```
feat: add inline assembly support
fix: resolve type inference bug in generics
docs: update README examples
test: add parser tests for structs
```

## Development Workflow

### Adding a Language Feature

1. Update `FERA_SPEC.md` with the proposed syntax/semantics
2. Add lexer tokens (if needed)
3. Extend parser to handle new syntax
4. Update type checker
5. Implement HIR lowering
6. Add codegen support
7. Write tests
8. Update documentation

### Testing Philosophy

- Unit tests for individual components
- Integration tests for end-to-end compilation
- Regression tests for bug fixes
- Performance benchmarks for optimization work

## Resources

- [Spec](../FERA_SPEC.md) - Language specification
- [README](../README.md) - Project overview
- [Discord/Matrix] - Community chat (TBD)

## License

By contributing, you agree that your contributions will be licensed under the MIT OR Apache-2.0 dual license.

