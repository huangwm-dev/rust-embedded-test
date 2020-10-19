#![no_std]
#![no_main]

use core::{
    panic::PanicInfo,
    ptr
};

static RODATA: &[u8] = b"Hello, world!";
static mut BSS: u8 = 0;
static mut DATA: u16 = 1;


#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> !
{
    loop {}
}

#[link_section = ".vector_table.reset_vector"]
#[no_mangle]
pub static __RESET_VECTOR: unsafe extern "C" fn() -> ! = reset_handler;

#[no_mangle]
pub unsafe extern "C" fn reset_handler() -> !
{
    extern "C" {
        static mut _sbss: u8; // Start of .bss section
        static mut _ebss: u8; // End of .bss section
        static mut _sdata: u8; // Start of .data section
        static mut _edata: u8; // End of .data section
        static _sidata: u8; // Start of .rodata section
    }

    let count = &_ebss as *const u8 as usize - &_sbss as *const u8 as usize;
    ptr::write_bytes(&mut _sbss as *mut u8, 0, count);

    let count = &_edata as *const u8 as usize - &_sdata as *const u8 as usize;
    ptr::copy_nonoverlapping(&_sidata as *const u8, &mut _sdata as *mut u8, count);

    extern "Rust" {
        fn main() -> !;
    }

   main()
}

#[no_mangle]
pub fn main() -> ! {
    let _x = RODATA;
    let _y = unsafe { &BSS };
    let _z = unsafe { &DATA };

    let mut _i = 0;
    loop {
        _i += 1;
    }
}
