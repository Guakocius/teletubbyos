#![no_std]
#![no_main]

use core::panic::PanicInfo;

const COM1: u16 = 0x3F8;

#[inline(always)]
unsafe fn outb(port: u16, val: u8) {
    core::arch::asm!(
        "out dx, al",
        in("dx") port,
        in("al") val,
        options(nomem, nostack, preserves_flags),
    );
}

#[inline(always)]
unsafe fn inb(port: u16) -> u8 {
    let mut val: u8;
    core::arch::asm!(
        "in al, dx",
        in("dx") port,
        out("al") val,
        options(nomem, nostack, preserves_flags),
    );
    val
}

unsafe fn serial_init() {
    outb(COM1 + 1, 0x00); // interrupts off
    outb(COM1 + 3, 0x80); // DLAB on
    outb(COM1 + 0, 0x03); // divisor low
    outb(COM1 + 1, 0x00); // divisor high
    outb(COM1 + 3, 0x03); // 8N1
    outb(COM1 + 2, 0xC7); // FIFO on, clear
    outb(COM1 + 4, 0x0B); // RTS/DSR
}

unsafe fn serial_write_byte(b: u8) {
    while (inb(COM1 + 5) & 0x20) == 0 {}
    outb(COM1, b);
}

unsafe fn serial_write_str(s: &str) {
    for &b in s.as_bytes() {
        if b == b'\n' {
            serial_write_byte(b'\r');
        }
        serial_write_byte(b);
    }
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    unsafe {
        serial_init();
        serial_write_str("TeletubbyOS kernel: booted.\n");
        serial_write_str("Status: Teletubbys still contained.\n");
    }
    loop {
        core::hint::spin_loop();
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    unsafe {
        serial_init();
        serial_write_str("\nKERNEL PANIC\n");
    }
    loop {
        core::hint::spin_loop();
    }
}
