#![no_std]
#![feature(asm)]
#![feature(llvm_asm)]
#![feature(start)]

use core::panic::PanicInfo;

mod vga;
mod asm;
mod fonts;

#[no_mangle]
fn hlt() {
    unsafe {
        asm!("hlt");
    }
}

#[no_mangle]
#[start]
pub extern "C" fn haribote_os() -> ! {
    use vga::{Screen, ScreenWriter};
    let mut screen = Screen::new();
    screen.init();

    let mut writer = ScreenWriter::new(screen, vga::Color::White, 10, 10);
    use core::fmt::Write;
    write!(writer, "ABCDD123==\nabc\n").unwrap();
    write!(writer, "afeaf afeia afeawfea afla aoijfeaf afjaifeaf").unwrap();

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

