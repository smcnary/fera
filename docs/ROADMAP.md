# Fera Roadmap

## v0.1.0 - Initial Bootstrap (Current)
**Target: Q1 2026**

- [x] Project structure
- [x] Basic lexer with token support
- [x] Recursive descent parser
- [x] AST definitions
- [x] Type checker skeleton
- [x] HIR (High-level IR)
- [x] LLVM IR codegen
- [x] CLI tool (build, run, check, clean)
- [ ] Core standard library
- [ ] Complete parser (all language features)
- [ ] Full type checking
- [ ] Error reporting with diagnostics
- [ ] x86_64 Linux support

## v0.2.0 - Core Language Features
**Target: Q2 2026**

- [ ] Structs, unions, enums
- [ ] Function pointers
- [ ] Arrays (fixed and flexible)
- [ ] Pointer arithmetic
- [ ] Type aliases (typedef)
- [ ] Const/volatile/restrict qualifiers
- [ ] Basic preprocessor (#define, #include)
- [ ] C FFI
- [ ] Static analysis (basic)

## v0.3.0 - Multi-Platform
**Target: Q3 2026**

- [ ] macOS (aarch64) support
- [ ] Windows (x86_64) support
- [ ] Cross-compilation
- [ ] Target specification
- [ ] Platform-specific stdlib

## v0.4.0 - Developer Experience
**Target: Q4 2026**

- [ ] Better error messages (codespan-reporting)
- [ ] Code formatter (`fera fmt`)
- [ ] LSP (Language Server Protocol)
- [ ] Syntax highlighting (VS Code, vim, etc.)
- [ ] Package manager basics
- [ ] Documentation generator

## v0.5.0 - Optimization & Safety
**Target: Q1 2027**

- [ ] Optimization passes
- [ ] LTO (Link-Time Optimization)
- [ ] AddressSanitizer integration
- [ ] UndefinedBehaviorSanitizer
- [ ] Bounds checking (debug mode)
- [ ] Static analyzer improvements

## v1.0.0 - Production Ready
**Target: Q2 2027**

- [ ] Complete language spec implementation
- [ ] Full stdlib (core + hosted)
- [ ] Stable ABI
- [ ] Performance on par with C
- [ ] Comprehensive test suite
- [ ] Complete documentation
- [ ] Package registry
- [ ] Self-hosting (compiler written in Fera)

## v1.1.0 - Advanced Features
**Target: Q4 2027**

- [ ] Atomics & memory ordering
- [ ] Lightweight generics (templates)
- [ ] Test harness (`fera test`)
- [ ] Inline assembly improvements
- [ ] Module system
- [ ] Lifetime hints (borrow-checker lite)

## v1.2.0 - Embedded Support
**Target: Q1 2028**

- [ ] ARM Cortex-M targets
- [ ] RISC-V support
- [ ] Freestanding mode
- [ ] Interrupt handlers
- [ ] MMIO abstractions
- [ ] Bare-metal stdlib

## v2.0.0 - Future Vision
**Target: TBD**

- [ ] Borrow-checker (Rust-like, opt-in)
- [ ] Async/await (coroutines)
- [ ] SIMD intrinsics
- [ ] JIT compilation mode
- [ ] Advanced type system features
- [ ] Formal verification support

## Community Milestones

- [ ] 100 GitHub stars
- [ ] 10 contributors
- [ ] First external project built with Fera
- [ ] Featured on language benchmarks
- [ ] Conference talk/paper

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for how to help achieve these goals!

