# Fera Quick Start Guide

## What is Fera?

Fera is a C-class systems programming language that combines the control and predictability of C with modern tooling and optional safety features.

## Installation

You'll need Rust and LLVM to build the Fera compiler:

```bash
# Install Rust if you don't have it
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Build Fera
cd /path/to/fera
make build

# Or with cargo
cargo build --release
```

The compiler will be at `target/release/fera`.

## Your First Program

Create `hello.fera`:

```c
export i32 main() {
    print("Hello, Fera!\n");
    return 0;
}
```

Compile and run:

```bash
./target/release/fera build hello.fera
./hello
```

Or in one step:

```bash
./target/release/fera run hello.fera
```

## Language Basics

### Types

```c
// Integers
i8, i16, i32, i64, isize      // Signed
u8, u16, u32, u64, usize      // Unsigned

// Floats
f32, f64

// Others
bool, char, void
```

### Functions

```c
i32 add(i32 a, i32 b) {
    return a + b;
}

export i32 main() {
    i32 result = add(5, 3);
    return 0;
}
```

### Structs

```c
struct Point {
    f32 x;
    f32 y;
};

Point make_point(f32 x, f32 y) {
    Point p;
    p.x = x;
    p.y = y;
    return p;
}
```

### Pointers

```c
void increment(i32* ptr) {
    *ptr = *ptr + 1;
}

i32 main() {
    i32 x = 10;
    increment(&x);  // x is now 11
    return 0;
}
```

### Arrays

```c
i32 sum(i32* arr, usize len) {
    i32 total = 0;
    for (usize i = 0; i < len; i = i + 1) {
        total = total + arr[i];
    }
    return total;
}
```

## CLI Commands

```bash
fera build <file>        # Compile to binary
fera run <file>          # Compile and run
fera check <file>        # Type-check only
fera clean               # Remove build artifacts
fera --help              # Show help
```

## Build Options

```bash
fera build main.fera --release          # Optimized build
fera build main.fera -O3                # Max optimization
fera build main.fera --link m           # Link libm
```

## Examples

Check the `examples/` directory:

- `hello.fera` - Hello World
- `fibonacci.fera` - Recursive Fibonacci
- `array_sum.fera` - Array operations
- `struct_example.fera` - Struct usage
- `pointers.fera` - Pointer operations

## Next Steps

- Read the [Language Spec](FERA_SPEC.md) for complete details
- Check out [CONTRIBUTING.md](docs/CONTRIBUTING.md) to help develop Fera
- See the [Roadmap](docs/ROADMAP.md) for planned features

## Getting Help

- **Issues:** Report bugs on GitHub
- **Spec Questions:** See FERA_SPEC.md
- **Contributing:** See docs/CONTRIBUTING.md

---

**Happy coding with Fera! 🦊**

