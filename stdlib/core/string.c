#include "core.h"
#include <string.h>
#include <ctype.h>

// String search functions
const char* str_chr(const char* s, char c) {
    return strchr(s, c);
}

const char* str_str(const char* haystack, const char* needle) {
    return strstr(haystack, needle);
}

// String concatenation (note: user must ensure dest has enough space)
char* str_cat(char* dest, const char* src) {
    return strcat(dest, src);
}

char* str_ncat(char* dest, const char* src, size_t n) {
    return strncat(dest, src, n);
}

// String comparison
int32_t str_cmp(const char* s1, const char* s2) {
    return strcmp(s1, s2);
}

int32_t str_ncmp(const char* s1, const char* s2, size_t n) {
    return strncmp(s1, s2, n);
}

// String copy
char* str_cpy(char* dest, const char* src) {
    return strcpy(dest, src);
}

char* str_ncpy(char* dest, const char* src, size_t n) {
    return strncpy(dest, src, n);
}

// String length
size_t str_len(const char* s) {
    return strlen(s);
}

size_t str_nlen(const char* s, size_t maxlen) {
    return strnlen(s, maxlen);
}

// Character classification
int32_t is_digit(int32_t c) {
    return isdigit(c);
}

int32_t is_alpha(int32_t c) {
    return isalpha(c);
}

int32_t is_alnum(int32_t c) {
    return isalnum(c);
}

int32_t is_space(int32_t c) {
    return isspace(c);
}

int32_t is_upper(int32_t c) {
    return isupper(c);
}

int32_t is_lower(int32_t c) {
    return islower(c);
}

// Character conversion
int32_t to_upper(int32_t c) {
    return toupper(c);
}

int32_t to_lower(int32_t c) {
    return tolower(c);
}

// Simple string to integer conversion
int32_t str_to_i32(const char* str) {
    int32_t result = 0;
    int32_t sign = 1;
    
    // Skip whitespace
    while (is_space(*str)) str++;
    
    // Handle sign
    if (*str == '-') {
        sign = -1;
        str++;
    } else if (*str == '+') {
        str++;
    }
    
    // Convert digits
    while (is_digit(*str)) {
        result = result * 10 + (*str - '0');
        str++;
    }
    
    return result * sign;
}

int64_t str_to_i64(const char* str) {
    int64_t result = 0;
    int64_t sign = 1;
    
    // Skip whitespace
    while (is_space(*str)) str++;
    
    // Handle sign
    if (*str == '-') {
        sign = -1;
        str++;
    } else if (*str == '+') {
        str++;
    }
    
    // Convert digits
    while (is_digit(*str)) {
        result = result * 10 + (*str - '0');
        str++;
    }
    
    return result * sign;
}

