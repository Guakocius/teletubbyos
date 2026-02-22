#![no_std]
#![no_main]

mod limine;

use core::panic::PanicInfo;
use limine::{FRAMEBUFFER_REQUEST, MEMMAP_REQUEST};

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    unsafe { fb_banner_panic(); }
    halt()
}

#[inline(always)]
fn halt() -> ! {
    loop {
        unsafe { core::arch::asm!("hlt"); }
    }
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    unsafe {
        let fb = first_fb().unwrap_or_else(|| halt());
        fb_clear(fb, 0x00101010);

        // "alive" bar
        fb_fill_rect(fb, 0, 0, fb.width as u32, 24, 0x00_2d_2d_2d);

        // crude text (block font)
        draw_word_teletubbyos(fb, 16, 6, 0x00_e0_e0_e0);

        // also draw a green square so you can’t miss it
        fb_fill_rect(fb, 16, 40, 64, 64, 0x00_00_ff_00);

        // If memmap response exists, draw a second square = "memmap OK"
        if !MEMMAP_REQUEST.response.is_null() {
            fb_fill_rect(fb, 96, 40, 64, 64, 0x00_00_80_ff);
        }
    }

    halt()
}

unsafe fn phys_to_virt(p: u64) -> *mut u8 {
    let off = (*limine::HHDM_REQUEST.response).offset;
    (p + off) as *mut u8
}

unsafe fn first_fb() -> Option<&'static limine::LimineFramebuffer> {
    if FRAMEBUFFER_REQUEST.response.is_null() {
        return None;
    }
    let resp = &*FRAMEBUFFER_REQUEST.response;
    if resp.framebuffer_count == 0 || resp.framebuffers.is_null() {
        return None;
    }
    let fb_ptr = *resp.framebuffers; // first framebuffer
    if fb_ptr.is_null() {
        None
    } else {
        Some(&*fb_ptr)
    }
}

unsafe fn fb_clear(fb: &limine::LimineFramebuffer, rgb: u32) {
    fb_fill_rect(fb, 0, 0, fb.width as u32, fb.height as u32, rgb);
}

unsafe fn fb_fill_rect(fb: &limine::LimineFramebuffer, x: u32, y: u32, w: u32, h: u32, rgb: u32) {
    // assume 32bpp for now (common in QEMU)
    // if bpp != 32, we still draw but it may look wrong; good enough for step-2 bringup.
    let bytes_per_px = (fb.bpp as u32 + 7) / 8;
    let pitch = fb.pitch as u32;
    let fb_phys = fb.address as u64;
    let fb_virt = phys_to_virt(fb_phys);
    let base = fb_virt;

    let max_x = (fb.width as u32).min(x.saturating_add(w));
    let max_y = (fb.height as u32).min(y.saturating_add(h));

    for yy in y..max_y {
        for xx in x..max_x {
            let off = yy * pitch + xx * bytes_per_px;
            let p = base.add(off as usize);
            // write as little-endian 0x00RRGGBB (works for typical XRGB8888)
            // if masks differ, colors may swap; still visible.
            core::ptr::write_volatile(p as *mut u32, rgb);
        }
    }
}

unsafe fn fb_banner_panic() {
    if let Some(fb) = first_fb() {
        fb_fill_rect(fb, 0, 0, fb.width as u32, 32, 0x00_00_00_ff); // red-ish
        fb_fill_rect(fb, 0, 32, fb.width as u32, 32, 0x00_ff_ff_ff);
    }
}

/* --- micro 5x7 glyphs for exactly "TELETUBBYOS" --- */

unsafe fn draw_word_teletubbyos(fb: &limine::LimineFramebuffer, x: u32, y: u32, col: u32) {
    let mut cx = x;
    for ch in b"TELETUBBYOS" {
        draw_glyph_5x7(fb, cx, y, *ch, col);
        cx += 6; // 5px glyph + 1px spacing
    }
}

unsafe fn draw_glyph_5x7(fb: &limine::LimineFramebuffer, x: u32, y: u32, ch: u8, col: u32) {
    // Each row: 5 bits used (MSB->left)
    let glyph: [u8; 7] = match ch {
        b'T' => [0b11111, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100],
        b'E' => [0b11111, 0b10000, 0b10000, 0b11110, 0b10000, 0b10000, 0b11111],
        b'L' => [0b10000, 0b10000, 0b10000, 0b10000, 0b10000, 0b10000, 0b11111],
        b'U' => [0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b01110],
        b'B' => [0b11110, 0b10001, 0b10001, 0b11110, 0b10001, 0b10001, 0b11110],
        b'Y' => [0b10001, 0b10001, 0b01010, 0b00100, 0b00100, 0b00100, 0b00100],
        b'O' => [0b01110, 0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b01110],
        b'S' => [0b01111, 0b10000, 0b10000, 0b01110, 0b00001, 0b00001, 0b11110],
        _ => [0, 0, 0, 0, 0, 0, 0],
    };

    for (row, bits) in glyph.iter().copied().enumerate() {
        for col_idx in 0..5u32 {
            let mask = 1u8 << (4 - col_idx);
            if (bits & mask) != 0 {
                fb_fill_rect(fb, x + col_idx, y + row as u32, 1, 1, col);
            }
        }
    }
}
