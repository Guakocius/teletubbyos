use crate::limine::*;
use crate::vga;

const PAGE_SIZE: u64 = 4096;

static mut NEXT_FREE: u64 = 0;
static mut END_FREE: u64 = 0;

pub unsafe fn init_from_memmap() {
    let resp = &*MEMMAP_REQUEST.response;

    // find first usable region big enough
    for i in 0..resp.entry_count {
        let e = &*(*resp.entries.add(i as usize));
        if e.typ == LIMINE_MEMMAP_USABLE && e.length >= PAGE_SIZE {
            NEXT_FREE = align_up(e.base, PAGE_SIZE);
            END_FREE = e.base + e.length;
            vga::write_string("\nPMM: using region base=");
            vga::write_hex_u64(e.base);
            vga::write_string(" len=");
            vga::write_hex_u64(e.length);
            vga::write_string("\n");
            return;
        }
    }

    vga::write_string("\nPMM: no usable memory found\n");
    loop { core::arch::asm!("hlt"); }
}

#[inline(always)]
const fn align_up(x: u64, a: u64) -> u64 {
    (x + (a - 1)) & !(a - 1)
}

// returns physical address of a 4KiB frame
pub unsafe fn alloc_frame() -> Option<u64> {
    if NEXT_FREE + PAGE_SIZE > END_FREE {
        return None;
    }
    let p = NEXT_FREE;
    NEXT_FREE += PAGE_SIZE;
    Some(p)
}
