#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>

// BMB Runtime Library
void bmb_println_i64(int64_t n) { printf("%ld\n", n); }
void bmb_print_i64(int64_t n) { printf("%ld", n); }
int64_t bmb_read_int() { int64_t n; scanf("%ld", &n); return n; }
void bmb_assert(int cond) { if (!cond) { fprintf(stderr, "Assertion failed!\n"); exit(1); } }
int64_t bmb_abs(int64_t n) { return n < 0 ? -n : n; }
int64_t bmb_min(int64_t a, int64_t b) { return a < b ? a : b; }
int64_t bmb_max(int64_t a, int64_t b) { return a > b ? a : b; }
double bmb_i64_to_f64(int64_t n) { return (double)n; }
int64_t bmb_f64_to_i64(double f) { return (int64_t)f; }

// v0.97: Character functions
int32_t bmb_chr(int64_t n) { return (int32_t)n; }
int64_t bmb_ord(int32_t c) { return (int64_t)c; }

// v0.97: String functions
void bmb_print_str(const char* s) { printf("%s", s); }
void bmb_println_str(const char* s) { printf("%s\n", s); }
int64_t bmb_str_len(const char* s) { int64_t len = 0; while (s[len]) len++; return len; }

// v0.98: Vector functions
// Layout: ptr[0] = capacity, ptr[1] = length, ptr[2...] = data
int64_t bmb_vec_new() {
    int64_t* vec = (int64_t*)malloc(10 * sizeof(int64_t));
    vec[0] = 8;  // capacity
    vec[1] = 0;  // length
    return (int64_t)vec;
}

int64_t bmb_vec_with_capacity(int64_t cap) {
    int64_t* vec = (int64_t*)malloc((cap + 2) * sizeof(int64_t));
    vec[0] = cap;  // capacity
    vec[1] = 0;    // length
    return (int64_t)vec;
}

void bmb_vec_push(int64_t vec_ptr, int64_t value) {
    int64_t* vec = (int64_t*)vec_ptr;
    int64_t cap = vec[0];
    int64_t len = vec[1];
    if (len >= cap) {
        // Grow: double capacity
        int64_t new_cap = cap * 2;
        int64_t* new_vec = (int64_t*)realloc(vec, (new_cap + 2) * sizeof(int64_t));
        new_vec[0] = new_cap;
        vec = new_vec;
    }
    vec[2 + len] = value;
    vec[1] = len + 1;
}

int64_t bmb_vec_pop(int64_t vec_ptr) {
    int64_t* vec = (int64_t*)vec_ptr;
    int64_t len = vec[1];
    if (len == 0) return 0;  // Empty vector
    vec[1] = len - 1;
    return vec[2 + len - 1];
}

int64_t bmb_vec_get(int64_t vec_ptr, int64_t index) {
    int64_t* vec = (int64_t*)vec_ptr;
    return vec[2 + index];
}

void bmb_vec_set(int64_t vec_ptr, int64_t index, int64_t value) {
    int64_t* vec = (int64_t*)vec_ptr;
    vec[2 + index] = value;
}

int64_t bmb_vec_len(int64_t vec_ptr) {
    int64_t* vec = (int64_t*)vec_ptr;
    return vec[1];
}

int64_t bmb_vec_cap(int64_t vec_ptr) {
    int64_t* vec = (int64_t*)vec_ptr;
    return vec[0];
}

void bmb_vec_free(int64_t vec_ptr) {
    free((void*)vec_ptr);
}

void bmb_vec_clear(int64_t vec_ptr) {
    int64_t* vec = (int64_t*)vec_ptr;
    vec[1] = 0;  // Reset length
}

// v0.99: String conversion functions
char* bmb_char_to_string(int32_t c) {
    char* s = (char*)malloc(5);  // UTF-8 max 4 bytes + null
    if (c < 0x80) {
        s[0] = (char)c;
        s[1] = '\0';
    } else if (c < 0x800) {
        s[0] = (char)(0xC0 | (c >> 6));
        s[1] = (char)(0x80 | (c & 0x3F));
        s[2] = '\0';
    } else if (c < 0x10000) {
        s[0] = (char)(0xE0 | (c >> 12));
        s[1] = (char)(0x80 | ((c >> 6) & 0x3F));
        s[2] = (char)(0x80 | (c & 0x3F));
        s[3] = '\0';
    } else {
        s[0] = (char)(0xF0 | (c >> 18));
        s[1] = (char)(0x80 | ((c >> 12) & 0x3F));
        s[2] = (char)(0x80 | ((c >> 6) & 0x3F));
        s[3] = (char)(0x80 | (c & 0x3F));
        s[4] = '\0';
    }
    return s;
}

char* bmb_int_to_string(int64_t n) {
    char* s = (char*)malloc(21);  // Max i64 is 20 digits + sign
    snprintf(s, 21, "%ld", (long)n);
    return s;
}

// Memory access functions
void bmb_store_i64(int64_t ptr, int64_t value) {
    *((int64_t*)ptr) = value;
}

int64_t bmb_load_i64(int64_t ptr) {
    return *((int64_t*)ptr);
}

// calloc wrapper (returns pointer as i64)
int64_t bmb_calloc(int64_t count, int64_t size) {
    return (int64_t)calloc((size_t)count, (size_t)size);
}

// Box convenience
int64_t bmb_box_new_i64(int64_t value) {
    int64_t* ptr = (int64_t*)malloc(sizeof(int64_t));
    *ptr = value;
    return (int64_t)ptr;
}

// v0.100: String concatenation
char* bmb_string_concat(const char* a, const char* b) {
    size_t len_a = 0, len_b = 0;
    while (a[len_a]) len_a++;
    while (b[len_b]) len_b++;
    char* result = (char*)malloc(len_a + len_b + 1);
    for (size_t i = 0; i < len_a; i++) result[i] = a[i];
    for (size_t i = 0; i < len_b; i++) result[len_a + i] = b[i];
    result[len_a + len_b] = '\0';
    return result;
}

// v0.46: Command-line argument support for CLI Independence
static int g_argc = 0;
static char** g_argv = NULL;

int64_t bmb_arg_count(void) {
    return (int64_t)g_argc;
}

char* bmb_get_arg(int64_t index) {
    if (index < 0 || index >= g_argc) {
        // Return empty string for out-of-bounds
        char* empty = (char*)malloc(1);
        empty[0] = '\0';
        return empty;
    }
    // Return a copy of the argument
    const char* arg = g_argv[index];
    size_t len = 0;
    while (arg[len]) len++;
    char* result = (char*)malloc(len + 1);
    for (size_t i = 0; i <= len; i++) result[i] = arg[i];
    return result;
}

// Entry point
int64_t bmb_user_main(void);
int main(int argc, char** argv) {
    g_argc = argc;
    g_argv = argv;
    return (int)bmb_user_main();
}
