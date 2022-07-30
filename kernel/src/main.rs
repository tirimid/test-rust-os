#![no_std]
#![no_main]

#[macro_use]
mod conio;

mod kcore;
mod memutil;
mod mathutil;
mod vga;
mod cpuio;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    vga::clear(vga::VgaColor::White, vga::VgaColor::Black);
    loop {
        vga::print_byte(b'h', vga::VgaColor::White, vga::VgaColor::Black);
        kcore::waste_cycles(80000);
    }
}
