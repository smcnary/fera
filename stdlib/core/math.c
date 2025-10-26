#include "core.h"
#include <math.h>

// Basic integer math functions
int32_t abs_i32(int32_t x) {
    return x < 0 ? -x : x;
}

int64_t abs_i64(int64_t x) {
    return x < 0 ? -x : x;
}

int32_t min_i32(int32_t a, int32_t b) {
    return a < b ? a : b;
}

int32_t max_i32(int32_t a, int32_t b) {
    return a > b ? a : b;
}

int64_t min_i64(int64_t a, int64_t b) {
    return a < b ? a : b;
}

int64_t max_i64(int64_t a, int64_t b) {
    return a > b ? a : b;
}

int32_t clamp_i32(int32_t value, int32_t min, int32_t max) {
    if (value < min) return min;
    if (value > max) return max;
    return value;
}

// Floating point math functions (wrappers around C math.h)
float sqrt_f32(float x) {
    return sqrtf(x);
}

double sqrt_f64(double x) {
    return sqrt(x);
}

float pow_f32(float base, float exp) {
    return powf(base, exp);
}

double pow_f64(double base, double exp) {
    return pow(base, exp);
}

float sin_f32(float x) {
    return sinf(x);
}

double sin_f64(double x) {
    return sin(x);
}

float cos_f32(float x) {
    return cosf(x);
}

double cos_f64(double x) {
    return cos(x);
}

float tan_f32(float x) {
    return tanf(x);
}

double tan_f64(double x) {
    return tan(x);
}

float log_f32(float x) {
    return logf(x);
}

double log_f64(double x) {
    return log(x);
}

float exp_f32(float x) {
    return expf(x);
}

double exp_f64(double x) {
    return exp(x);
}

float floor_f32(float x) {
    return floorf(x);
}

double floor_f64(double x) {
    return floor(x);
}

float ceil_f32(float x) {
    return ceilf(x);
}

double ceil_f64(double x) {
    return ceil(x);
}

float round_f32(float x) {
    return roundf(x);
}

double round_f64(double x) {
    return round(x);
}

float abs_f32(float x) {
    return fabsf(x);
}

double abs_f64(double x) {
    return fabs(x);
}

// Integer algorithms
int32_t gcd_i32(int32_t a, int32_t b) {
    a = abs_i32(a);
    b = abs_i32(b);
    while (b != 0) {
        int32_t temp = b;
        b = a % b;
        a = temp;
    }
    return a;
}

int64_t gcd_i64(int64_t a, int64_t b) {
    a = abs_i64(a);
    b = abs_i64(b);
    while (b != 0) {
        int64_t temp = b;
        b = a % b;
        a = temp;
    }
    return a;
}

int32_t lcm_i32(int32_t a, int32_t b) {
    if (a == 0 || b == 0) return 0;
    return abs_i32(a * b) / gcd_i32(a, b);
}

int64_t lcm_i64(int64_t a, int64_t b) {
    if (a == 0 || b == 0) return 0;
    return abs_i64(a * b) / gcd_i64(a, b);
}

