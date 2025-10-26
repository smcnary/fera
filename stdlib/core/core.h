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

// String search and manipulation
const char* str_chr(const char* s, char c);
const char* str_str(const char* haystack, const char* needle);
char* str_cat(char* dest, const char* src);
char* str_ncat(char* dest, const char* src, size_t n);
int32_t str_cmp(const char* s1, const char* s2);
int32_t str_ncmp(const char* s1, const char* s2, size_t n);
char* str_cpy(char* dest, const char* src);
char* str_ncpy(char* dest, const char* src, size_t n);
size_t str_len(const char* s);
size_t str_nlen(const char* s, size_t maxlen);

// Character classification
int32_t is_digit(int32_t c);
int32_t is_alpha(int32_t c);
int32_t is_alnum(int32_t c);
int32_t is_space(int32_t c);
int32_t is_upper(int32_t c);
int32_t is_lower(int32_t c);
int32_t to_upper(int32_t c);
int32_t to_lower(int32_t c);

// String to number conversion
int32_t str_to_i32(const char* str);
int64_t str_to_i64(const char* str);

// I/O - Basic string output
void print(const char* s);
void println(const char* s);

// I/O - Typed output
void print_i32(int32_t value);
void print_i64(int64_t value);
void print_u32(uint32_t value);
void print_u64(uint64_t value);
void print_f32(float value);
void print_f64(double value);
void print_bool(int32_t value);
void print_ptr(const void* ptr);

void println_i32(int32_t value);
void println_i64(int64_t value);
void println_u32(uint32_t value);
void println_u64(uint64_t value);
void println_f32(float value);
void println_f64(double value);
void println_bool(int32_t value);
void println_ptr(const void* ptr);

// Math - Integer functions
int32_t abs_i32(int32_t x);
int64_t abs_i64(int64_t x);
int32_t min_i32(int32_t a, int32_t b);
int32_t max_i32(int32_t a, int32_t b);
int64_t min_i64(int64_t a, int64_t b);
int64_t max_i64(int64_t a, int64_t b);
int32_t clamp_i32(int32_t value, int32_t min, int32_t max);
int32_t gcd_i32(int32_t a, int32_t b);
int64_t gcd_i64(int64_t a, int64_t b);
int32_t lcm_i32(int32_t a, int32_t b);
int64_t lcm_i64(int64_t a, int64_t b);

// Math - Floating point functions
float sqrt_f32(float x);
double sqrt_f64(double x);
float pow_f32(float base, float exp);
double pow_f64(double base, double exp);
float sin_f32(float x);
double sin_f64(double x);
float cos_f32(float x);
double cos_f64(double x);
float tan_f32(float x);
double tan_f64(double x);
float log_f32(float x);
double log_f64(double x);
float exp_f32(float x);
double exp_f64(double x);
float floor_f32(float x);
double floor_f64(double x);
float ceil_f32(float x);
double ceil_f64(double x);
float round_f32(float x);
double round_f64(double x);
float abs_f32(float x);
double abs_f64(double x);

// Panic/abort
[[noreturn]] void panic(const char* msg);

#endif // FERA_CORE_H

