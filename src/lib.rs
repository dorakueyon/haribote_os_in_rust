#![no_std]
#![feature(asm)]
#![feature(llvm_asm)]
#![feature(start)]

use core::panic::PanicInfo;

mod vga;
mod asm;

#[no_mangle]
fn hlt() {
    unsafe {
        asm!("hlt");
    }
}

#[no_mangle]
#[start]
pub extern "C" fn haribote_os() -> ! {
    vga::set_palette();
    loop {
        hlt()
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        hlt()
    }
}

