#![no_std]
#![no_main]

mod kcore;
mod conio;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    kcore::hang();
}
