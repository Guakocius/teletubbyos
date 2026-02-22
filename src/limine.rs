#![allow(dead_code)]

#[repr(C)]
pub struct LimineMemmapEntry {
    pub base: u64,
    pub length: u64,
    pub typ: u64,
}

#[repr(C)]
pub struct LimineMemmapResponse {
    pub revision: u64,
    pub entry_count: u64,
    pub entries: *const *const LimineMemmapEntry,
}

#[repr(C)]
pub struct LimineMemmapRequest {
    pub id: [u64; 4],
    pub revision: u64,
    pub response: *const LimineMemmapResponse,
}

unsafe impl Sync for LimineMemmapRequest {}

#[used]
#[link_section = ".limine_requests"]
pub static KERNEL_ADDRESS_REQUEST: LimineKernelAddressRequest = ...;

#[used]
#[link_section = ".limine_requests"]
pub static MEMMAP_REQUEST: LimineMemmapRequest = ...; /*LimineMemmapRequest {
    id: [
        0x67cf3d9d378a806f,
        0xe304acdfc50c3c62,
        0x4b6fe466aade04ce,
        0x0c7b0c0f4d7c6d32,
    ],
    revision: 0,
    response: core::ptr::null(),
};*/

#[repr(C)]
pub struct LimineFramebuffer {
    pub address: *mut u8,
    pub width: u64,
    pub height: u64,
    pub pitch: u64,

    pub bpp: u16,
    pub memory_model: u8,
    pub red_mask_size: u8,
    pub red_mask_shift: u8,
    pub green_mask_size: u8,
    pub green_mask_shift: u8,
    pub blue_mask_size: u8,
    pub blue_mask_shift: u8,
    pub unused: [u8; 7],
}

#[repr(C)]
pub struct LimineFramebufferResponse {
    pub revision: u64,
    pub framebuffer_count: u64,
    pub framebuffers: *const *const LimineFramebuffer,
}

#[repr(C)]
pub struct LimineFramebufferRequest {
    pub id: [u64; 4],
    pub revision: u64,
    pub response: *const LimineFramebufferResponse,
}

unsafe impl Sync for LimineFramebufferRequest {}

#[used]
#[link_section = ".limine_requests"]
pub static FRAMEBUFFER_REQUEST: LimineFramebufferRequest = ...; /*LimineFramebufferRequest {
    id: [
        0xc7b1dd30df4c8b88,
        0x0a82e883a194f07b,
        0x6d74c4d1c57f16aa,
        0x9b70d1f6b30c6f39,
    ],
    revision: 0,
    response: core::ptr::null(),
};*/

#[repr(C)]
pub struct LimineHhdmResponse {
    pub revision: u64,
    pub offset: u64,
}

#[repr(C)]
pub struct LimineHhdmRequest {
    pub id: [u64; 4],
    pub revision: u64,
    pub response: *const LimineHhdmResponse,
}

unsafe impl Sync for LimineHhdmRequest {}

#[used]
#[link_section = ".limine_requests"]
pub static HHDM_REQUEST: LimineHhdmRequest = ...; /*LimineHhdmRequest {
    id: [
        0x48dcf1cb8ad2b852,
        0x63984e959a98244b,
        0x84b840f49d7a0d0d,
        0x5a59a7cbdc4e4f4a,
    ],
    revision: 0,
    response: core::ptr::null(),
};*/

/* Memmap types (Limine) */
pub const LIMINE_MEMMAP_USABLE: u64 = 0;
pub const LIMINE_MEMMAP_RESERVED: u64 = 1;
pub const LIMINE_MEMMAP_ACPI_RECLAIMABLE: u64 = 2;
pub const LIMINE_MEMMAP_ACPI_NVS: u64 = 3;
pub const LIMINE_MEMMAP_BAD_MEMORY: u64 = 4;
pub const LIMINE_MEMMAP_BOOTLOADER_RECLAIMABLE: u64 = 5;
pub const LIMINE_MEMMAP_KERNEL_AND_MODULES: u64 = 6;
pub const LIMINE_MEMMAP_FRAMEBUFFER: u64 = 7;
