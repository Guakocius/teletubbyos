#include <stddef.h>

// Minimal bare-metal stubs for Scala Native runtime

extern char _heap_start;
extern char _heap_end;

static char* heap_ptr = &_heap_start;

void* malloc(size_t size) {
    void* ptr = heap_ptr;
    heap_ptr += (size + 7) & ~7; // align to 8 bytes
    if (heap_ptr > &_heap_end) return (void*)0;
    return ptr;
}

void free(void* ptr) { /* bump allocator - no free */ }

void* realloc(void* ptr, size_t size) {
    void* new_ptr = malloc(size);
    return new_ptr;
}

void* calloc(size_t nmemb, size_t size) {
    void* ptr = malloc(nmemb * size);
    // zero it
    char* p = ptr;
    for (size_t i = 0; i < nmemb * size; i++) p[i] = 0;
    return ptr;
}
