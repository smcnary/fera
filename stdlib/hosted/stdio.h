#ifndef FERA_STDIO_H
#define FERA_STDIO_H

// Fera Hosted Standard Library - File I/O
// Available on Linux/Windows/macOS

#include <stdio.h>

// Re-export standard C file operations
typedef FILE File;

#define STDIN  stdin
#define STDOUT stdout
#define STDERR stderr

File* fopen(const char* filename, const char* mode);
int fclose(File* stream);
size_t fread(void* ptr, size_t size, size_t nmemb, File* stream);
size_t fwrite(const void* ptr, size_t size, size_t nmemb, File* stream);
int fseek(File* stream, long offset, int whence);
long ftell(File* stream);
int fflush(File* stream);

int printf(const char* format, ...);
int fprintf(File* stream, const char* format, ...);
int sprintf(char* str, const char* format, ...);
int snprintf(char* str, size_t size, const char* format, ...);

#endif // FERA_STDIO_H

