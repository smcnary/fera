# Fera Compiler Setup Guide

## Current Status

The Fera language compiler has been updated with the following improvements:

### ✅ Completed
- Code generation with LLVM integration
- Standard library compilation and linking
- Built-in function declarations (print, println)
- Lexer improvements (comment filtering, EOF handling)
- Comprehensive documentation
- Example programs
- Build verification script

### ⏳ Pending
- Cargo compilation verification (requires Rust toolchain installation)

## Prerequisites

Before you can build the Fera compiler, you need:

1. **Rust Toolchain** (1.70 or later)
   - Cargo (build tool and package manager)
   - rustc (Rust compiler)
   - rustfmt (code formatter)
   - clippy (linter)

2. **LLVM** (17.0 or later)
   - Required for code generation
   - Usually provided by system package manager

3. **C Compiler**
   - GCC or Clang
   - Required for linking standard library

## Installation Steps

### Step 1: Install Rust

```bash
# Install rustup (Rust installer)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Restart your shell or run:
source $HOME/.cargo/env

# Verify installation
cargo --version
rustc --version
```

### Step 2: Install LLVM (if needed)

**On macOS:**
```bash
brew install llvm@17
```

**On Ubuntu/Debian:**
```bash
sudo apt-get update
sudo apt-get install llvm-17-dev libllvm17 llvm-17
```

**On Fedora/RHEL:**
```bash
sudo dnf install llvm-devel
```

### Step 3: Clone and Build Fera

```bash
# Navigate to the Fera directory
cd /Users/seanmcnary/repos/fera/fera

# Build the compiler
make build

# Or using cargo directly:
cargo build --release
```

### Step 4: Verify the Build

```bash
# Run the verification script
./verify_build.sh

# Or manually test:
./target/release/fera --version
./target/release/fera run examples/hello.fera
```

### Step 5: Install (Optional)

```bash
# Install to ~/.cargo/bin/
make install

# Or:
cargo install --path .

# Verify installation
fera --version
```

## Post-Installation

### Running Examples

```bash
# Hello World
fera run examples/hello.fera

# Fibonacci
fera run examples/fibonacci.fera

# Comprehensive example
fera run examples/comprehensive.fera
```

### Creating Your First Program

Create a file `myprogram.fera`:

```c
export i32 main() {
    print("Hello from my program!\n");
    return 0;
}
```

Build and run:

```bash
fera build myprogram.fera
./myprogram

# Or in one step:
fera run myprogram.fera
```

## Development Workflow

### Building for Development

```bash
# Debug build (faster compile, slower runtime)
cargo build

# The binary will be at: target/debug/fera
```

### Running Tests

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_basic

# Run with output
cargo test -- --nocapture
```

### Code Quality

```bash
# Format code
cargo fmt

# Run linter
cargo clippy -- -D warnings

# Check without building
cargo check
```

### Making Changes

1. Edit source files in `src/`
2. Run `cargo check` to verify syntax
3. Run `cargo test` to verify correctness
4. Run `cargo clippy` to check style
5. Run `cargo fmt` to format code
6. Build with `cargo build`

## Project Structure

```
fera/
├── src/                    # Compiler source code (Rust)
│   ├── main.rs            # Entry point
│   ├── lexer/             # Tokenization
│   ├── parser/            # Parsing
│   ├── ast/               # AST definitions
│   ├── types/             # Type checking
│   ├── hir/               # High-level IR
│   ├── codegen/           # LLVM code generation
│   └── cli/               # CLI commands
├── stdlib/                # Standard library (C)
│   └── core/              # Core functions
│       ├── core.h         # Header file
│       └── print.c        # I/O functions
├── examples/              # Example programs
├── tests/                 # Integration tests
├── docs/                  # Documentation
├── Cargo.toml            # Rust dependencies
├── Makefile              # Build automation
└── verify_build.sh       # Build verification
```

## Troubleshooting

### Cargo Not Found

If you get "cargo: command not found":

1. Install Rust: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
2. Restart your shell or: `source $HOME/.cargo/env`
3. Verify: `cargo --version`

### LLVM Not Found

If you get LLVM-related errors:

1. Install LLVM for your platform (see Installation Steps)
2. Set LLVM_SYS_170_PREFIX environment variable if needed:
   ```bash
   export LLVM_SYS_170_PREFIX=/usr/local/opt/llvm@17
   ```

### Linker Errors

If you get linker errors:

1. Make sure you have a C compiler: `cc --version` or `gcc --version`
2. On macOS, install Xcode Command Line Tools: `xcode-select --install`
3. On Linux, install build-essential: `sudo apt-get install build-essential`

### Permission Errors

If you get permission errors:

```bash
# Make scripts executable
chmod +x verify_build.sh

# Or for the Makefile
chmod +x Makefile
```

## What's Next?

After successful installation:

1. **Read Documentation**
   - [Language Spec](FERA_SPEC.md) - Complete language reference
   - [Quick Start](QUICKSTART.md) - Quick introduction
   - [Build Status](BUILD_STATUS.md) - Current capabilities

2. **Try Examples**
   - Run all examples in `examples/` directory
   - Modify them and experiment

3. **Write Code**
   - Create your own Fera programs
   - Explore language features

4. **Contribute**
   - See [CONTRIBUTING.md](docs/CONTRIBUTING.md)
   - Check [ROADMAP.md](docs/ROADMAP.md) for planned features

## Getting Help

- **Issues:** Report bugs on GitHub
- **Documentation:** See docs/ directory
- **Examples:** Check examples/ directory
- **Community:** Join discussions on GitHub

## Recent Changes

See [CHANGES_SUMMARY.md](CHANGES_SUMMARY.md) for recent improvements and fixes.

## Version Information

- **Fera Version:** 0.1.0 (Alpha)
- **Rust Edition:** 2021
- **LLVM Version:** 17.0
- **License:** MIT OR Apache-2.0

---

**Ready to build? Run: `make build`** 🚀

