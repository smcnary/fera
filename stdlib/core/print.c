#include "core.h"
#include <unistd.h>
#include <string.h>
#include <stdio.h>

void print(const char* s) {
    if (s) {
        write(STDOUT_FILENO, s, strlen(s));
    }
}

void println(const char* s) {
    print(s);
    write(STDOUT_FILENO, "\n", 1);
}

void panic(const char* msg) {
    write(STDERR_FILENO, "PANIC: ", 7);
    if (msg) {
        write(STDERR_FILENO, msg, strlen(msg));
    }
    write(STDERR_FILENO, "\n", 1);
    __builtin_trap();
}

// Formatted print functions for different types
void print_i32(int32_t value) {
    char buffer[32];
    snprintf(buffer, sizeof(buffer), "%d", value);
    print(buffer);
}

void print_i64(int64_t value) {
    char buffer[32];
    snprintf(buffer, sizeof(buffer), "%lld", (long long)value);
    print(buffer);
}

void print_u32(uint32_t value) {
    char buffer[32];
    snprintf(buffer, sizeof(buffer), "%u", value);
    print(buffer);
}

void print_u64(uint64_t value) {
    char buffer[32];
    snprintf(buffer, sizeof(buffer), "%llu", (unsigned long long)value);
    print(buffer);
}

void print_f32(float value) {
    char buffer[64];
    snprintf(buffer, sizeof(buffer), "%g", value);
    print(buffer);
}

void print_f64(double value) {
    char buffer[64];
    snprintf(buffer, sizeof(buffer), "%g", value);
    print(buffer);
}

void print_bool(int32_t value) {
    print(value ? "true" : "false");
}

void print_ptr(const void* ptr) {
    char buffer[32];
    snprintf(buffer, sizeof(buffer), "%p", ptr);
    print(buffer);
}

// Println variants
void println_i32(int32_t value) {
    print_i32(value);
    print("\n");
}

void println_i64(int64_t value) {
    print_i64(value);
    print("\n");
}

void println_u32(uint32_t value) {
    print_u32(value);
    print("\n");
}

void println_u64(uint64_t value) {
    print_u64(value);
    print("\n");
}

void println_f32(float value) {
    print_f32(value);
    print("\n");
}

void println_f64(double value) {
    print_f64(value);
    print("\n");
}

void println_bool(int32_t value) {
    print_bool(value);
    print("\n");
}

void println_ptr(const void* ptr) {
    print_ptr(ptr);
    print("\n");
}

