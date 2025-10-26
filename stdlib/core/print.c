#include "core.h"
#include <unistd.h>
#include <string.h>

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

