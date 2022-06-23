#![no_std]
#![no_main]

use core::panic::PanicInfo;

static HELLO_MSG: &[u8] = b"hello world";

const VGA_PTR: *mut u8 = 0xb8000 as *mut u8;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    for (i, &byte) in HELLO_MSG.iter().enumerate() {
        unsafe {
            *VGA_PTR.offset(i as isize * 2) = byte;
            *VGA_PTR.offset(i as isize * 2 + 1) = 0xf;
        }
    }
    
    loop {}
}
