# 🦊 Fera
> **Fera** is a modern low-level programming language forged for systems programming, embedded development, and high-performance computing.

Fera aims to combine the **predictability and control of C** with a **cleaner syntax, stronger safety levers,** and a **fast, modern toolchain.**  
It’s built to stay close to the metal — no garbage collector, no runtime overhead, just raw performance.

---

## 🚀 Features
- **Zero runtime:** No hidden allocations, no GC — what you write is what runs.  
- **Ahead-of-time compiled:** Produces native binaries for Linux, Windows, macOS, and embedded targets.  
- **C interoperability:** Seamless FFI with existing C codebases.  
- **Modern tooling:** Fast incremental builds, diagnostics, and formatting.  
- **Safety on demand:** Optional bounds, UB, and memory checks in debug.  

---

## ⚙️ Example

```c
// hello.fera
export int main() {
    print("Hello, Fera!\n");
    return 0;
}

fera build hello.fera
./hello
