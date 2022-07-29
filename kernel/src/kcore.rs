use core::panic::PanicInfo;
use core::arch::asm;

#[panic_handler]
pub fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

pub fn hang() -> ! {
    loop {
        unsafe {
            asm!("hlt");
        }
    }
}

pub fn waste_cycles(cycle_cnt: usize) {
    for _ in 0..cycle_cnt {
        unsafe {
            asm!("nop");
        }
    }
}
