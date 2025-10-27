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

### Standard Library Functions

Fera includes a comprehensive standard library with 70+ functions:

**Math Functions:**
```c
// Integer math
i32 abs_i32(i32 x);
i32 max_i32(i32 a, i32 b);
i32 min_i32(i32 a, i32 b);
i32 gcd_i32(i32 a, i32 b);

// Floating point math
f64 sqrt_f64(f64 x);
f64 pow_f64(f64 base, f64 exp);
f64 sin_f64(f64 x);
f64 cos_f64(f64 x);
```

**String Functions:**
```c
// String operations
i32 str_cmp(char* s1, char* s2);
char* str_chr(char* s, char c);
i32 str_to_i32(char* str);

// Character classification
i32 is_digit(i32 c);
i32 is_alpha(i32 c);
```

**I/O Functions:**
```c
// Basic I/O
void print(char* s);
void println(char* s);

// Formatted output
void print_i32(i32 value);
void print_f64(f64 value);
void println_i32(i32 value);
```

See [`stdlib/core/core.h`](stdlib/core/core.h) for the complete list.

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

- **[Quick Start](QUICKSTART.md)** - Get started quickly
- **[Language Spec](FERA_SPEC.md)** - Complete language specification
- **[Build Status](BUILD_STATUS.md)** - Current build status and recent updates
- **[Contributing Guide](docs/CONTRIBUTING.md)** - How to contribute
- **[Roadmap](docs/ROADMAP.md)** - Future plans and milestones

---

## 🏗️ Project Status

Fera is in **early alpha**. The compiler is functional but many features are still being implemented.

**Current Status:**
- ✅ Lexer and parser with span tracking
- ✅ Type system with comprehensive checking
- ✅ LLVM IR codegen with optimizations
- ✅ Complete CLI tooling
- ✅ Extensive stdlib (70+ functions)
- ✅ Error reporting with helpful messages
- ❌ Package manager (planned)

See the [Roadmap](docs/ROADMAP.md) for detailed progress.

---

## 🗺️ Vision & Roadmap

### Core Language & Toolchain (Tier 0)

**Compiler (fera):**
- AOT compiler with cross-compilation support
- Link-time optimization (LTO)
- Sanitizers (address, thread, UB)
- Debug info (DWARF/PDB)

**Standard Libraries:**
- `feracore` - Freestanding: memory, slices, fmt, math
- `ferastd` - Hosted: fs, net, time, threads
- `feraintrin` - SIMD, atomics, special registers

**Build System & Package Manager:**
- `ferabuild` - Project graph, targets, cross profiles
- `ferapkg` - Package manager with lockfile and semver
- `ferahub` - Registry (index site + search + docs hosting)

**Developer Experience:**
- `fera fmt` - Code formatter
- `fera lint` - Lints and clippy-like hints
- `fera test` - Unit tests + snapshot tests
- `fera bench` - Microbenchmark harness
- `fera-lsp` - Language server (IDE features)
- `fera dbg` - Debugger (CLI wrapper for lldb/gdb + pretty printers)
- Profiler - Sampling + perf counters (BPF/RDTSC)
- Static analyzer - UB, lifetime heuristics, aliasing warnings

### Operating System Platform (Tier 1)

**FeraOS Kernel:**
- Scheduler, virtual memory, syscalls
- VFS (Virtual File System)
- Device drivers

**Userspace Base:**
- `feraclibc` - C lib shim + POSIX-lite for portability
- `ferad` - Init system + service manager
- `fsh` - Shell
- `ferautils` - Coreutils

**Drivers (Priority):**
- Serial, console, timer, keyboard
- AHCI, NVMe
- e1000e/virtio-net, virtio-blk
- Framebuffer

**Filesystems:**
- RAMFS → FAT32 → ext2/3 (read-only first)
- tarfs for initrd

**Networking:**
- IPv4, ARP, ICMP, UDP, TCP
- DHCP client
- DNS resolver

**Packaging:**
- `ferapk` - Binary packages, repo metadata, signatures

### Runtimes & Interop (Tier 2)

**FeraCLR:**
- Interpreter + AOT compilation
- Minimal class library

**FFI Layers:**
- Stable C ABI
- Headers import/export
- Bindgen-like tool for C → Fera bindings

**WebAssembly:**
- `fera-wasm` backend for sandboxed plugins
- WASI support

**Scripting:**
- FeraScript (config/templating)
- Lua port

### Frameworks & Libraries (Tier 3)

**Networking:**
- `feranet` - Async reactor
- `ferahttp` - HTTP/1.1 server/client
- WebSocket support

**Storage:**
- `feradb` - Embedded KV store (LSM-lite)
- SQLite bindings
- ORM-style helpers

**Crypto:**
- `feracrypto` - Hashes, AEAD, TLS v2 roadmap

**Concurrency:**
- `ferafibers` - Fibers/coroutines with I/O integration

**Graphics/UI (Future):**
- Framebuffer UI toolkit
- Font rasterizer

**Observability:**
- `feralog` - Logging
- `ferametrics` - Metrics
- `feratrace` - Tracing (text, JSON, OTLP export)

### DevOps, Distribution & Supply Chain (Tier 4)

**Build Farm & Releases:**
- Deterministic builds
- Build cache (S3/minio)
- Cross-compiled artifacts per triple

**Reproducible Builds:**
- Pinned toolchains
- SOURCE_DATE_EPOCH
- SBOMs (Software Bill of Materials)

**Signing & Security:**
- `ferasign` - Key management + package signing
- Provenance attestations

**Container/VM Images:**
- FeraOS cloud images (QEMU/virt, KVM)
- PXE boot support
- Minimal rootfs

**CI/CD:**
- GitHub/GitLab action templates
- Pipeline: lint → test → build → sign → publish

**Crash Reporting:**
- Symbol server
- Minidump support

### Documentation & Education (Tier 5)

**Documentation Site (docs.fera.dev):**
- Language book
- OS handbook
- API documentation

**Examples Repository:**
- Sample applications
- Driver examples
- Kernel tutorials
- Networking demos

**Interactive Learning:**
- Browser REPL (Wasm backend)
- Shareable code gists

**Guides:**
- "From C to Fera"
- "Porting a Driver"
- "Writing a TCP Server"

---

## 📅 Minimal Viable Roadmap (12-15 Months)

### Phase A — Bootstrap (Months 0-3)
- ✅ `fera` compiler (x86_64/aarch64) + `feracore`
- ✅ `fera fmt`, `fera test`
- FeraOS boots to shell over serial with RAMFS
- Cooperative task scheduler
- `ferabuild` can compile multi-crate workspace
- Seed documentation published

### Phase B — Developer-Usable (Months 4-6)
- `ferapkg` + Ferahub MVP (index + docs render)
- `fera-lsp`, basic debugger wrappers
- Sanitizer integration
- TCP/UDP stack + HTTP server (`ferahttp`)
- AHCI/virtio-blk, e1000e/virtio-net drivers
- ext2 filesystem (read-only)
- CI pipelines + signed release artifacts

### Phase C — Production Shape (Months 7-10)
- Preemptive scheduler
- Per-process virtual memory
- ELF loader (user mode)
- POSIX-lite (open/read/write/pipe/fork subset)
- Observability stack (log/metrics/trace)
- Package signing + SBOMs
- Binary repo mirroring
- FeraCLR interpreter + AOT for plugins

### Phase D — Ecosystem Expansion (Months 11-15)
- Fibers, async I/O
- TLS support
- DNS, HTTP client
- SQLite bindings, KV store
- Reproducible builds, cache farm
- Symbol server
- Wasm backend (optional)
- Browser playground
- "1.0" documentation + stability guarantees

---

## 📦 Repository Organization

```
org/
├─ fera               # Compiler + tools
├─ feracore           # Freestanding core lib
├─ ferastd            # Hosted std lib
├─ feraintrin         # Intrinsics/arch shims
├─ ferabuild          # Build system/driver
├─ ferapkg            # Package manager
├─ ferahub            # Registry + site
├─ fera-lsp           # Language server
├─ fera-dbg           # Debugger/pretty printers
├─ ferafmt            # Formatter (if separate)
├─ feralint           # Linter rules
├─ feraos             # OS kernel/userspace
├─ feranet            # Reactor + net stack
├─ ferahttp           # HTTP server/client
├─ feracrypto         # Crypto primitives
├─ feradb             # KV/SQLite bindings
├─ ferametrics        # Metrics library
├─ feratrace          # Tracing library
├─ feraclr            # Common language runtime
├─ examples           # End-to-end sample apps
└─ website-docs       # Docs + landing
```

---

## 🎯 Quality Standards

### Language & Packages
- ✅ Semver + compatibility policy
- ✅ Lockfiles, checksums, content-addressable cache
- ✅ Cross-target dependencies (features, cfg guards)
- ✅ Build scripts (host tools) with sandboxing

### Tooling Quality
- ✅ <100ms formatter on typical files
- ✅ LSP: diagnostics, code actions, go-to-def, rename, format-on-save
- ✅ Unit tests + golden tests for compiler/OS
- ✅ Fuzzers on parser, IR passes, net stack

### Security & Supply Chain
- ✅ Signed releases, reproducible builds, SBOM
- ✅ Static analysis & sanitizers on CI
- ✅ Kernel hardening flags, CFI with LTO
- ✅ Memory init opt-in (-ftrivial-auto-init)

### OS/Userland UX
- ✅ `ferad` service unit format (TOML/YAML)
- ✅ Logs to /var/log, journaling option
- ✅ Coreutils subset (cat, ls, cp, mv, echo, ps, kill)
- ✅ Networking CLI (ip, ifconfig, route, dns)

---

## 🚀 Starter Verticals

**Web Appliance:**
- FeraOS + `ferahttp` + file server + metrics dashboard

**Edge/Embedded:**
- NVMe logger, UDP telemetry, OTA updates via signed packages

**Plugin Host:**
- Application embedding FeraCLR or Wasm for safe extensions

**Network Services:**
- DNS responder, syslog collector, NTP client

---

## 🏛️ Governance & Community

- **RFC Process:** Lightweight ADRs for language/OS changes
- **Stability Guarantees:** Edition model or LTS tags
- **Contribution Guide:** Code style, commit tags, CLA (if needed)
- **Bug Bounty:** Especially for kernel/networking

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
