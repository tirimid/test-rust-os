use core::panic::PanicInfo;
use core::arch::asm;
use crate::vga;
use crate::conio;
use vga::VgaColor;

pub fn hang() -> ! {
    loop {
        unsafe {
            asm!("hlt");
        }
    }
}

#[panic_handler]
pub fn panic(info: &PanicInfo) -> ! {
    unsafe {
        conio::CONOUT.flush_fg_col = VgaColor::White;
        conio::CONOUT.flush_bg_col = VgaColor::Red;
        vga::clear(conio::CONOUT.flush_fg_col, conio::CONOUT.flush_bg_col);
    }
    println!("{info}");
    hang();
}

pub fn waste_cycles(cycle_cnt: usize) {
    for _ in 0..cycle_cnt {
        unsafe {
            asm!("nop");
        }
    }
}
