use core::ptr::write_volatile;

const VGA: usize = 0xB8000;
const W: usize = 80;
const H: usize = 25;

static mut ROW: usize = 0;
static mut COL: usize = 0;
static mut ATTR: u8 = 0x0F; // weiß auf schwarz

#[repr(transparent)]
struct Cell(u16);

fn idx(r: usize, c: usize) -> usize { r * W + c }

#[no_mangle]
pub extern "C" fn vga_set_color(fg: u8, bg: u8) {
    unsafe { ATTR = (bg << 4) | (fg & 0x0F); }
}

#[no_mangle]
pub extern "C" fn vga_clear() {
    for i in 0..(W * H) {
        unsafe { write_volatile((VGA as *mut Cell).add(i), Cell(((0x07 as u16) << 8) | 0x20)); }
    }
    unsafe { ROW = 0; COL = 0; }
}

fn newline() {
    unsafe {
        COL = 0;
        ROW += 1;
        if ROW >= H { ROW = 0; } // simpel, später scroll
    }
}

#[no_mangle]
pub extern "C" fn vga_putc(ch: u8) {
    unsafe {
        if ch == b'\n' { newline(); return; }
        let at = idx(ROW, COL);
        let cell = Cell(((ATTR as u16) << 8) | ch as u16);
        write_volatile((VGA as *mut Cell).add(at), cell);
        COL += 1;
        if COL >= W { newline(); }
    }
}

#[no_mangle]
pub extern "C" fn vga_write(ptr: *const u8, len: usize) {
    if ptr.is_null() { return; }
    for i in 0..len {
        unsafe { vga_putc(*ptr.add(i)); }
    }
}

pub fn write_string(s: &str) {
    vga_write(s.as_bytes().as_ptr(), s.len());
}

pub fn write_hex_u64(mut x: u64) {
    write_string("0x");
    for i in (0..16).rev() {
        let nibble = ((x >> (i * 4)) & 0xF) as u8;
        let c = match nibble {
            0..=9 => b'0' + nibble,
            _ => b'a' + (nibble - 10),
        };
        vga_putc(c);
    }
}

pub fn write_dec_u64(mut x: u64) {
    if x == 0 {
        vga_putc(b'0');
        return;
    }
    let mut buf = [0u8; 20];
    let mut i = 0usize;
    while x > 0 {
        buf[i] = b'0' + (x % 10) as u8;
        x /= 10;
        i += 1;
    }
    while i > 0 {
        i -= 1;
        vga_putc(buf[i]);
    }
}
