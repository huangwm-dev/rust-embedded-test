#![no_std]
#![no_main]

use core::{
    panic::PanicInfo,
    ptr::{read, write_volatile},
    mem::zeroed
};

static RODATA: &[u8] = b"Hello, World";
static mut DATA: u16 = 1;
static mut BSS: u8 = 0;

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
        static mut _sdata: u32; // Start of .data section
        static mut _edata: u32; // End of .data section
        static mut _sidata: u32; // Start of .rodata section

        static mut _sbss: u32;  // start of .bss
        static mut _ebss: u32;  // end of .bss
    }

    // Initialize data section
    let mut sdata: *mut u32 = &mut _sdata as *mut u32;
    let edata: *mut u32 = &mut _edata as *mut u32;
    let mut sidata: *mut u32 = &mut _sidata as *mut u32;

    while sdata < edata
    {
        write_volatile(sdata, read(sidata));
        sdata = sdata.offset(1);
        sidata = sidata.offset(1);
    }

    // Initialize BSS section
    let mut sbss: *mut u32 = &mut _sbss as *mut u32;
    let ebss: *mut u32 = &mut _ebss as *mut u32;
    while sbss < ebss
    {
        write_volatile(sbss, zeroed());
        sbss = sbss.offset(1);
    }

    extern "Rust" {
        fn main() -> !;
    }

   main()
}

#[no_mangle]
pub fn main() -> ! {

    let mut _i = 0;
    let _x = RODATA;
    let _y = unsafe { &BSS };
    let _z = unsafe { &DATA };
    loop {
        _i += 1;
    }
}
