// BMB Runtime Library
// Provides basic I/O functions for BMB programs

#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>

// Print i64 without newline
void bmb_print_i64(int64_t x) {
    printf("%lld", (long long)x);
}

// Print i64 with newline
void bmb_println_i64(int64_t x) {
    printf("%lld\n", (long long)x);
}

// Print f64 without newline
void bmb_print_f64(double x) {
    printf("%g", x);
}

// Print f64 with newline
void bmb_println_f64(double x) {
    printf("%g\n", x);
}

// Print boolean
void bmb_println_bool(int b) {
    printf("%s\n", b ? "true" : "false");
}

// Assert condition
void bmb_assert(int cond, const char* msg) {
    if (!cond) {
        fprintf(stderr, "Assertion failed: %s\n", msg);
        exit(1);
    }
}

// Panic with message
void bmb_panic(const char* msg) {
    fprintf(stderr, "panic: %s\n", msg);
    exit(1);
}

// ===================================================
// Bootstrap Runtime Functions
// These match the declarations in bootstrap/llvm_ir.bmb
// Using bmb_ prefix to avoid conflicts with stdlib/Windows
// ===================================================

// Undefine Windows macros that conflict with our function names
#ifdef min
#undef min
#endif
#ifdef max
#undef max
#endif

// println(i64) - Print i64 with newline (bootstrap version)
void println(int64_t x) {
    printf("%lld\n", (long long)x);
}

// print(i64) - Print i64 without newline
void print(int64_t x) {
    printf("%lld", (long long)x);
}

// read_int() - Read i64 from stdin
int64_t read_int(void) {
    int64_t x;
    if (scanf("%lld", (long long*)&x) != 1) {
        fprintf(stderr, "Error: failed to read integer\n");
        exit(1);
    }
    return x;
}

// assert(i1) - Assert condition is true
void assert(int cond) {
    if (!cond) {
        fprintf(stderr, "Assertion failed\n");
        exit(1);
    }
}

// abs(i64) - Absolute value
int64_t bmb_abs(int64_t x) {
    return x < 0 ? -x : x;
}

// min(i64, i64) - Minimum of two values
int64_t min(int64_t a, int64_t b) {
    return a < b ? a : b;
}

// max(i64, i64) - Maximum of two values
int64_t max(int64_t a, int64_t b) {
    return a > b ? a : b;
}
