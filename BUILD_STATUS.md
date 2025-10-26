# Fera Build Status

## Recent Updates (2025-10-26)

### Completed Improvements

1. **Standard Library Integration**
   - Added automatic compilation of `stdlib/core/print.c`
   - Integrated stdlib linking into the build process
   - Stdlib object files are automatically compiled and linked with Fera programs

2. **Built-in Function Declarations**
   - Added `print(char*)` function declaration in codegen
   - Added `println(char*)` function declaration in codegen
   - Functions are automatically available to all Fera programs

3. **Lexer Improvements**
   - Comments (line and block) are now properly filtered out
   - Preprocessor directives are skipped during tokenization
   - EOF token is now automatically added to the token stream

4. **Code Generation Fixes**
   - Fixed variable loading to use correct LLVM type (pointee type)
   - Fixed void function call handling in expression context
   - Improved error handling for function calls

### Build Process

The Fera compiler build process consists of several stages:

```
Source Code (.fera)
    ↓
  Lexer (tokenization)
    ↓
  Parser (AST generation)
    ↓
  Type Checker (semantic analysis)
    ↓
  HIR Builder (lower to High-level IR)
    ↓
  Code Generator (LLVM IR generation)
    ↓
  LLVM (object file generation)
    ↓
  Linker (link with stdlib and produce executable)
```

### How to Build the Compiler

```bash
# Build in release mode
make build

# Or using cargo directly
cargo build --release

# The binary will be at: target/release/fera
```

### How to Build Fera Programs

```bash
# Build a Fera program
fera build examples/hello.fera

# Build and run
fera run examples/hello.fera

# Build with optimizations
fera build examples/hello.fera --release

# Build with specific optimization level
fera build examples/hello.fera -O3
```

### Current Capabilities

The Fera compiler can currently:

- ✅ Lex and parse Fera source code
- ✅ Type check programs
- ✅ Generate LLVM IR
- ✅ Compile to native machine code
- ✅ Link with C standard library
- ✅ Support basic types (integers, floats, booleans, pointers)
- ✅ Support control flow (if, while, for)
- ✅ Support functions and function calls
- ✅ Support structs, unions, enums
- ✅ Support basic operators (arithmetic, comparison, logical, bitwise)

### Standard Library Functions

Currently available built-in functions:

- `void print(char* s)` - Print a string to stdout
- `void println(char* s)` - Print a string with newline to stdout

### Example Programs

#### Hello World
```c
// hello.fera
export i32 main() {
    print("Hello, Fera!\n");
    return 0;
}
```

#### Fibonacci
```c
// fibonacci.fera
i32 fib(i32 n) {
    if (n <= 1) {
        return n;
    }
    return fib(n - 1) + fib(n - 2);
}

export i32 main() {
    i32 n = 10;
    i32 result = fib(n);
    return 0;
}
```

### Known Limitations

1. **Limited stdlib** - Only basic I/O functions are available
2. **No package manager** - Planned for future releases
3. **Limited error messages** - Error reporting can be improved
4. **No type inference** - All types must be explicitly declared
5. **No generics yet** - Planned for future releases

### Development Environment

**Prerequisites:**
- Rust 1.70+
- LLVM 17.0+
- C compiler (GCC or Clang)

**Setup:**
```bash
# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Setup development environment
make dev-setup

# Run tests
make test

# Run linter
make lint

# Format code
make fmt
```

### File Structure

```
fera/
├── src/              # Compiler source (Rust)
│   ├── lexer/        # Tokenization
│   ├── parser/       # AST generation
│   ├── ast/          # AST definitions
│   ├── types/        # Type checking
│   ├── hir/          # High-level IR
│   ├── codegen/      # LLVM IR generation
│   ├── cli/          # CLI commands
│   └── main.rs       # Entry point
├── stdlib/           # Standard library (C)
│   └── core/         # Core functionality
│       ├── core.h    # Core header
│       └── print.c   # I/O functions
├── examples/         # Example programs
├── tests/            # Integration tests
└── docs/             # Documentation
```

### Next Steps

To continue development:

1. Expand standard library with more functions
2. Improve error reporting with better diagnostics
3. Add more example programs
4. Implement remaining CLI commands (test, fmt, doc)
5. Add support for modules and imports
6. Implement generics/templates
7. Add unsafe blocks for low-level operations
8. Improve optimization passes

### Testing

Run the test suite:
```bash
# Run all tests
make test

# Run specific test
cargo test test_basic

# Run with verbose output
cargo test -- --nocapture
```

### Contributing

See [CONTRIBUTING.md](docs/CONTRIBUTING.md) for guidelines on how to contribute to Fera.

### License

Dual-licensed under MIT or Apache-2.0. See LICENSE files for details.

