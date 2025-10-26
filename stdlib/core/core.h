#ifndef FERA_CORE_H
#define FERA_CORE_H

// Fera Core Standard Library
// Minimal always-available functionality

#include <stddef.h>
#include <stdint.h>

// Type aliases for clarity
typedef size_t usize;
typedef ptrdiff_t isize;

// Memory management
void* malloc(size_t size);
void* calloc(size_t num, size_t size);
void* realloc(void* ptr, size_t new_size);
void free(void* ptr);

void* memcpy(void* dest, const void* src, size_t n);
void* memset(void* s, int c, size_t n);
void* memmove(void* dest, const void* src, size_t n);
int memcmp(const void* s1, const void* s2, size_t n);

// String operations
size_t strlen(const char* s);
char* strcpy(char* dest, const char* src);
int strcmp(const char* s1, const char* s2);
char* strncpy(char* dest, const char* src, size_t n);
size_t strnlen(const char* s, size_t maxlen);

// I/O (minimal)
void print(const char* s);
void println(const char* s);

// Panic/abort
[[noreturn]] void panic(const char* msg);

#endif // FERA_CORE_H

