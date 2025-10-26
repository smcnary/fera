# Fera — A C-class Systems Programming Language

## 1) Vision & Goals
**Intent:** A portable, predictable, ahead-of-time compiled language for OS kernels, embedded, drivers, and perf-critical libraries.  

**Primary goals:**
- **Direct hardware control:** raw pointers, manual memory mgmt, inline asm.
- **Predictable layout & ABI:** C-compatible data layout & FFI.
- **Fast builds:** sub-second incremental compiles for medium modules.
- **Zero runtime:** no GC, optional tiny runtime stubs only.
- **Safety levers:** opt-in checks (`-fsanitize`, bounds in debug).

**Non-goals (v1):** exceptions, classes/OO, generational GC, reflection, concurrency runtime.

---

## 2) Language Surface (v1)

### 2.1 Lexical & Tokens
- **Encodings:** UTF-8 source; identifiers `[A-Za-z_][A-Za-z0-9_]*`.
- **Numerics:** `0x`, `0o`, `0b`, suffixes: `u`, `l`, `ul`. Floats: `1.0f`, `1.0`.
- **Comments:** `// line`, `/* block */`.
- **Whitespace:** ignored except to separate tokens.
- **Preprocessor (minimal):** `#define`, `#include`, `#if/#elif/#else/#endif`, `#pragma once`. (Token-pasting omitted in v1.)

### 2.2 Types
- **Integer:** `i8,i16,i32,i64,isize` and unsigned `u8…usize`.
- **Float:** `f32,f64`.
- **Bool/Char:** `bool`, `char` (8-bit, implementation-defined signedness), optional `wchar_t`.
- **Pointers:** `T*`, `const T*`, `volatile T*`, `restrict`.
- **Arrays:** `T[n]` (fixed), `T[]` (in param contexts).
- **Struct/Union:** C-compatible; packed via `[[packed(N)]]`.
- **Enums:** default backing `int`, override with `enum : u8 { … }`.
- **Function types:** `T (*)(Args)`; default `cdecl`, support `stdcall`/`fastcall` attributes.
- **Qualifiers:** `const`, `volatile`, `restrict`, `alignas(N)`.
- **Aliases:** `typedef` (v1). Optional `using` shorthand.

### 2.3 Expressions & Statements
- C-like precedence; pointer arithmetic on `T*` and `ptrdiff_t`.
- Control flow: `if/else`, `switch`, `for`, `while`, `do`, `break/continue`, labels & `goto`.
- Initialization: C99 designated `.(field)=value`.
- **No exceptions.** Errors via return codes or `[[noreturn]] panic(const char*)` in debug.

### 2.4 Functions, Linkage, Modules
- **Files = translation units.**
- **Linkage:** `internal` (like `static`), `export` (like `extern`).
- **Headers:** `.h` interfaces; `.fera` sources.
- **Attributes:** `[[noreturn]]`, `[[cold]]`, `[[hot]]`, `[[always_inline]]`, `[[noinline]]`, `[[deprecated("msg")]]`, `[[interrupt(irq)]]` (embedded).
- **Inline asm:** `asm volatile ("…": outputs : inputs : clobbers);` with backend constraints.

### 2.5 Memory Model & UB Policy
- **Model:** C11-like effective type + strict aliasing (toggle with `-fno-strict-aliasing`).
- **Alignment:** natural; misaligned access is target-defined; type-punning via `memcpy` is well-defined.
- **Atomics (v1.1):** `_Atomic(T)` and `atomic_{load,store,fetch_*}` with memory orders.
- **UB catalog:** OOB, UAF, null deref, signed overflow (wrap only with `-fwrapv`), data races, strict-aliasing violations. Document sanitizer mapping.

### 2.6 Generics (lightweight, v1.1)
- Macro-style or header-templated:
  ```c
  #define VEC(T) struct { T* data; size_t len, cap; }
  ```
- Instantiation at use-site. Errors in generated code point back to macro.
- Future: type-checked templates with constraints.

---

## 3) Compiler Architecture

### 3.1 Frontend (Lexer → Parser → AST → HIR)
- **Lexer:** UTF-8 → tokens (incl. preprocessor tokens).
- **Preprocessor:** expand `#define`, `#include` (w/ include guards), evaluate `#if`.
- **Parser:** hand-written recursive descent or LR. Produce typed AST.
- **HIR (High-level IR):** desugar, name resolution, type inference, borrow-check stubs (v1.1).

### 3.2 Middle-end (LLVM IR generation)
- Lower HIR → LLVM IR modules.
- Inline trivial functions; dead-code elim; constant folding.
- Link-time optimizations (LTO) for release builds.

### 3.3 Backend (Machine code)
- LLVM → native `.o` files.
- Support targets: `x86_64-linux-gnu`, `aarch64-linux-gnu`, `x86_64-windows-msvc`, `aarch64-apple-darwin`, bare-metal ARM (Cortex-M).
- Cross-compilation via target triples.

### 3.4 Build System
- **fera build** orchestrates incremental compilation:
  - Parse `fera.toml` manifest (dependencies, targets).
  - Fingerprint `.fera` sources; recompile changed modules.
  - Cache `.o` & IR for incremental builds.
- **fera run**, **fera test**, **fera clean**.

---

## 4) Standard Library (stdlib)

### 4.1 Core (minimal, always available)
- **Types:** `size_t`, `ptrdiff_t`, `intptr_t`, fixed-width integers.
- **Memory:** `malloc`, `calloc`, `realloc`, `free`, `memcpy`, `memset`, `memmove`, `memcmp`.
- **String:** `strlen`, `strcpy`, `strcmp`, `strncpy` (unsafe), `strnlen` (bounded).
- **I/O (minimal):** `print(const char*)`, `println(const char*)` (syscall wrappers or libc).
- **Panic:** `[[noreturn]] panic(const char* msg)` — prints & aborts.

### 4.2 Hosted OS stdlib (Linux/Windows/macOS)
- **File I/O:** `fopen`, `fclose`, `fread`, `fwrite`, `fseek`, `ftell`.
- **Process:** `exit`, `abort`, `atexit`.
- **Time:** `clock`, `time`, `localtime` (POSIX subset).
- **Math:** `sqrt`, `sin`, `cos`, `pow`, etc. (link `-lm`).

### 4.3 Embedded / Freestanding
- No heap by default; user provides allocator or static buffers.
- **Startup:** `_start` → `main`, initialize `.data`/`.bss`.
- **Interrupt vectors:** architecture-specific (ARM: NVIC setup).

---

## 5) Memory Safety Features (Opt-in, v1.1)

### 5.1 Bounds Checking (Debug mode)
- Array accesses via `arr[i]` insert runtime check: `if (i >= len) panic("OOB")`.
- Toggle: `-fbounds-check` / `-fno-bounds-check`.

### 5.2 Sanitizers
- **AddressSanitizer (ASan):** heap UAF, OOB, double-free.
- **UndefinedBehaviorSanitizer (UBSan):** signed overflow, null deref, misaligned access.
- **ThreadSanitizer (TSan):** data races (v1.1 with atomics).
- Compiler flag: `-fsanitize=address,undefined`.

### 5.3 Static Analysis (v1.1)
- Lint for uninitialized vars, unused results of `[[nodiscard]]` funcs.
- Lifetime analysis (similar to Rust borrow-checker, but opt-in).

---

## 6) Foreign Function Interface (FFI)

### 6.1 C ABI Compatibility
- Fera structs/enums match C layout (no field reordering by default).
- Function calling conventions: default `cdecl`, attributes for `stdcall`, `fastcall`.
- `extern "C"` linkage (use `export` for exported symbols).

### 6.2 Interop Example
**C header (`math.h`):**
```c
double sqrt(double x);
```

**Fera code:**
```c
extern double sqrt(double x);

export int main() {
    double result = sqrt(16.0);
    print("sqrt(16) = ");
    // ... (formatted print TBD)
    return 0;
}
```

### 6.3 Linking
- Link against system libs: `fera build --link m` (for libm).
- Static/dynamic linking controlled by linker flags.

---

## 7) Toolchain Commands

### 7.1 `fera build [file|project]`
- Compiles `.fera` sources → native binary.
- Options: `--release`, `--target <triple>`, `--link <lib>`, `-O<0-3>`.

### 7.2 `fera run [file|project]`
- Builds + executes the binary.
- Pass args: `fera run -- arg1 arg2`.

### 7.3 `fera test`
- Discovers & runs `test_*` functions (v1.1 test harness).
- Reports pass/fail, runtime.

### 7.4 `fera fmt [file]`
- Format source code (style: Allman braces, 4-space indent by default, configurable).

### 7.5 `fera check`
- Type-check & lint without code generation (fast feedback).

### 7.6 `fera clean`
- Remove build artifacts (`.o`, binaries, cache).

### 7.7 `fera doc` (v1.1)
- Generate HTML docs from `///` doc comments.

---

## 8) Project Structure & Manifest

### 8.1 `fera.toml` Manifest
```toml
[package]
name = "myproject"
version = "0.1.0"
authors = ["Alice <alice@example.com>"]

[target]
type = "executable"  # or "library", "staticlib"
entry = "main.fera"

[dependencies]
# Future: package registry
libc = { version = "1.0", link = true }

[build]
opt-level = 2
debug-info = true
```

### 8.2 Directory Layout
```
myproject/
├── fera.toml
├── src/
│   ├── main.fera
│   ├── util.fera
│   └── util.h        # optional header
├── include/          # public headers for libs
└── build/            # artifacts (gitignored)
    ├── myproject     # binary
    └── *.o
```

---

## 9) Error Handling Conventions

### 9.1 Return Codes
```c
enum Status : i32 {
    OK = 0,
    ERR_NULL_PTR = -1,
    ERR_OOB = -2,
    ERR_IO = -3,
};

Status read_file(const char* path, u8* buf, size_t* out_len);
```

### 9.2 Optional Types (v1.1 stdlib)
```c
typedef struct {
    bool has_value;
    T value;
} Option_T;

Option_int divide(int a, int b) {
    if (b == 0) return (Option_int){ .has_value = false };
    return (Option_int){ .has_value = true, .value = a / b };
}
```

### 9.3 Panic (Abort)
```c
[[noreturn]] void panic(const char* msg) {
    // print to stderr, dump stack trace (if debug), abort()
}
```

---

## 10) Roadmap & Versioning

### v1.0 (MVP)
- Core language features (sections 2.1–2.5).
- LLVM backend for x86_64/AArch64 (Linux, macOS, Windows).
- Minimal stdlib (section 4.1).
- Basic toolchain (`build`, `run`, `clean`).
- C FFI.

### v1.1
- Atomics & memory orders.
- Lightweight generics (macro-based).
- Test harness, doc gen.
- Optional bounds-checking, sanitizers.
- Static analyzer (uninitialized vars, lifetime hints).

### v1.2
- Type-checked templates/generics.
- Module system (beyond single-file).
- Package manager (fetch deps from registry).
- Embedded toolchain (ARM Cortex-M, RISC-V).

### v2.0 (Future)
- Borrow-checker (opt-in Rust-like lifetimes).
- Async/await (zero-cost coroutines).
- SIMD intrinsics.
- JIT compilation mode (LLVM OrcJIT).

---

## 11) Language Comparisons

| Feature | Fera | C | Rust | Zig |
|---------|------|---|------|-----|
| **Memory Safety** | Opt-in (sanitizers) | Manual | Enforced (borrow-checker) | Opt-in |
| **Generics** | Macro/templates (v1.1) | Macros | Parametric | comptime |
| **Runtime** | None | Minimal | None (std) | None |
| **ABI Stability** | C-compatible | Yes | Not guaranteed | C-compatible |
| **Build Speed** | Fast (incremental) | Fast | Slow (monomorphization) | Fast |
| **Error Handling** | Return codes/panic | Return codes | Result<T,E> | Error unions |
| **Inline ASM** | Yes | Yes | Yes | Yes |
| **Preprocessor** | Minimal (#define, #include) | Full | None | None (comptime) |

---

## 12) Example Programs

### 12.1 Hello World
```c
// hello.fera
export int main() {
    print("Hello, Fera!\n");
    return 0;
}
```
**Build:**
```bash
fera build hello.fera
./hello
```

### 12.2 Pointer Arithmetic
```c
export int sum_array(const i32* arr, size_t len) {
    i32 total = 0;
    for (size_t i = 0; i < len; i++) {
        total += arr[i];
    }
    return total;
}
```

### 12.3 Struct & Designated Init
```c
struct Vec3 {
    f32 x, y, z;
};

export Vec3 origin() {
    return (Vec3){ .x = 0.0f, .y = 0.0f, .z = 0.0f };
}
```

### 12.4 Inline Assembly (x86_64)
```c
export u64 rdtsc() {
    u32 lo, hi;
    asm volatile ("rdtsc" : "=a"(lo), "=d"(hi));
    return ((u64)hi << 32) | lo;
}
```

### 12.5 Embedded Blink LED (ARM Cortex-M)
```c
#define RCC_AHB1ENR (*(volatile u32*)0x40023830)
#define GPIOA_MODER (*(volatile u32*)0x40020000)
#define GPIOA_ODR   (*(volatile u32*)0x40020014)

export int main() {
    RCC_AHB1ENR |= (1 << 0);        // Enable GPIOA clock
    GPIOA_MODER &= ~(3 << (5*2));
    GPIOA_MODER |= (1 << (5*2));    // Set PA5 as output
    
    while (true) {
        GPIOA_ODR ^= (1 << 5);      // Toggle LED
        for (volatile int i = 0; i < 1000000; i++);
    }
    return 0;
}
```

---

## 13) Implementation Notes

### 13.1 Compiler Bootstrapping
- Phase 1: Write Fera compiler in Rust/C++ targeting LLVM.
- Phase 2: Self-host (rewrite compiler in Fera once stable).

### 13.2 Testing Strategy
- **Unit tests:** parser, type-checker, codegen (LLVM IR diffing).
- **Integration tests:** compile & run `.fera` snippets, check stdout.
- **Sanitizer tests:** verify ASan/UBSan catch errors.
- **Cross-compilation tests:** build for all targets, run on QEMU.

### 13.3 Performance Targets
- Compile speed: <1s for 10K SLOC module (incremental).
- Runtime perf: within 5% of equivalent C (with same opts).

---

## 14) Open Questions & Design Decisions

1. **Preprocessor scope:** Keep minimal (`#define`, `#include`) or remove entirely (Zig-style `@import`, `comptime`)?
2. **Generics syntax:** Stick with macros or invest in type-checked templates early?
3. **Module system:** File-based (v1) vs. explicit `mod` declarations (Rust-like)?
4. **Error handling evolution:** Introduce `Result<T,E>` enum or stick with return codes?
5. **Borrow-checker:** Opt-in (Rust-lite) or full enforcement (v2.0)?
6. **Concurrency:** Expose raw threads (pthreads/WinAPI) or provide async runtime (v2.0)?

---

## 15) License & Governance

- **Language spec:** CC-BY-4.0 (this document).
- **Compiler & stdlib:** MIT/Apache-2.0 dual-license (TBD).
- **Governance:** Benevolent dictator (initial) → steering committee (community-driven).

---

## 16) Resources & Community

- **Website:** fera-lang.org (TBD)
- **Repository:** github.com/fera-lang/fera
- **Chat:** Discord/Matrix (community)
- **RFCs:** Propose language changes via GitHub issues/PRs.

---

**Last Updated:** 2025-10-26  
**Spec Version:** 0.1.0-draft

