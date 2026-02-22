#include <stdint.h>

void serial_outb(uint16_t port, uint8_t value) {
    __asm__ volatile ("out %0, %1" : : "a"(value), "Nd"(port));
}

uint8_t serial_inb(uint16_t port) {
    uint8_t ret;
    __asm__ volatile ("in %1, %0" : "=a"(ret) : "Nd"(port));
    return ret;
}
