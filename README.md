# 🦊 Fera
> **Fera** is a modern low-level programming language forged for systems programming, embedded development, and high-performance computing.

Fera aims to combine the **predictability and control of C** with a **cleaner syntax, stronger safety levers,** and a **fast, modern toolchain.**  
It's built to stay close to the metal — no garbage collector, no runtime overhead, just raw performance.

[![Build Status](https://img.shields.io/badge/status-alpha-orange)](https://github.com/fera-lang/fera)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue)](LICENSE)

---

## 🚀 Features
- **Zero runtime:** No hidden allocations, no GC — what you write is what runs.  
- **Ahead-of-time compiled:** Produces native binaries for Linux, Windows, macOS, and embedded targets.  
- **C interoperability:** Seamless FFI with existing C codebases.  
- **Modern tooling:** Fast incremental builds, diagnostics, and formatting.  
- **Safety on demand:** Optional bounds, UB, and memory checks in debug.  

---

## 📦 Installation

### Prerequisites
- Rust 1.70+ (for building the compiler)
- LLVM 17.0+
- A C compiler (GCC or Clang)

### Build from Source

```bash
git clone https://github.com/fera-lang/fera.git
cd fera
make build

# Or using cargo directly
cargo build --release
```

The `fera` binary will be available at `target/release/fera`.

### Install System-Wide

```bash
make install
# Or: cargo install --path .
```

---

## ⚙️ Quick Start

### Hello World

Create a file `hello.fera`:

```c
// hello.fera
export i32 main() {
    print("Hello, Fera!\n");
    return 0;
}
```

Build and run:

```bash
fera build hello.fera
./hello
```

Or use `fera run` to build and execute in one step:

```bash
fera run hello.fera
```

### More Examples

**Fibonacci:**
```c
i32 fib(i32 n) {
    if (n <= 1) return n;
    return fib(n - 1) + fib(n - 2);
}

export i32 main() {
    i32 result = fib(10);
    return result;
}
```

**Structs and Pointers:**
```c
struct Vec3 {
    f32 x;
    f32 y;
    f32 z;
};

f32 vec3_dot(Vec3 a, Vec3 b) {
    return a.x * b.x + a.y * b.y + a.z * b.z;
}

export i32 main() {
    Vec3 v1 = { .x = 1.0f, .y = 2.0f, .z = 3.0f };
    Vec3 v2 = { .x = 4.0f, .y = 5.0f, .z = 6.0f };
    f32 dot = vec3_dot(v1, v2);
    return 0;
}
```

See the [`examples/`](examples/) directory for more examples.

---

## 🛠️ CLI Commands

```bash
fera build [file]          # Build a Fera program
fera run [file]            # Build and run
fera check [file]          # Type-check without building
fera fmt [files...]        # Format source code
fera test                  # Run tests
fera clean                 # Remove build artifacts
fera doc                   # Generate documentation
fera --help                # Show help
```

### Build Options

```bash
fera build hello.fera --release        # Optimized build
fera build hello.fera -O3              # Optimization level
fera build hello.fera --target <triple> # Cross-compile
fera build hello.fera --link m         # Link against libm
```

---

## 📚 Documentation

- **[Language Spec](FERA_SPEC.md)** - Complete language specification
- **[Contributing Guide](docs/CONTRIBUTING.md)** - How to contribute
- **[Roadmap](docs/ROADMAP.md)** - Future plans and milestones

---

## 🏗️ Project Status

Fera is in **early alpha**. The compiler is functional but many features are still being implemented.

**Current Status:**
- ✅ Lexer and parser
- ✅ Type system foundation
- ✅ LLVM IR codegen
- ✅ Basic CLI tooling
- ⚠️ Limited stdlib
- ⚠️ Error reporting (in progress)
- ❌ Package manager (planned)

See the [Roadmap](docs/ROADMAP.md) for detailed progress.

---

## 🧪 Development

### Build & Test

```bash
make dev          # Build in debug mode
make test         # Run tests
make lint         # Run clippy
make fmt          # Format code
make validate     # Run all checks
```

### Project Structure

```
fera/
├── src/           # Compiler source (Rust)
│   ├── lexer/     # Tokenization
│   ├── parser/    # AST generation
│   ├── types/     # Type checking
│   ├── hir/       # High-level IR
│   ├── codegen/   # LLVM IR generation
│   └── cli/       # CLI tools
├── stdlib/        # Standard library
├── examples/      # Example programs
├── tests/         # Integration tests
└── docs/          # Documentation
```

---

## 🌟 Why Fera?

| Feature | C | Rust | Zig | **Fera** |
|---------|---|------|-----|----------|
| Zero runtime | ✅ | ✅ | ✅ | ✅ |
| C ABI compatible | ✅ | ⚠️ | ✅ | ✅ |
| Memory safety | ❌ | ✅ (enforced) | ⚠️ (opt-in) | ⚠️ (opt-in) |
| Generics | ❌ | ✅ | ✅ (comptime) | ✅ (templates) |
| Learning curve | Low | High | Medium | **Low** |
| Build speed | Fast | Slow | Fast | **Fast** |
| Preprocessor | Full | ❌ | ❌ | Minimal |

**Fera's niche:** For developers who want C-level control with modern ergonomics, optional safety, and fast iteration cycles.

---

## 🤝 Contributing

Contributions are welcome! See [CONTRIBUTING.md](docs/CONTRIBUTING.md) for guidelines.

**Ways to help:**
- Report bugs and request features
- Improve documentation
- Add examples
- Implement language features
- Write tests

---

## 📜 License

Dual-licensed under:
- MIT License ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
- Apache License 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)

Choose whichever works best for your project.

---

## 🙏 Acknowledgments

Fera is inspired by:
- **C** - The foundation of systems programming
- **Rust** - Memory safety and modern tooling
- **Zig** - Comptime and simplicity
- **LLVM** - World-class compiler infrastructure

---

**Built with ❤️ for systems programmers who miss the simplicity of C but crave modern tools.**
