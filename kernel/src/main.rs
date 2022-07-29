#![no_std]
#![no_main]

mod kcore;
mod memutil;
mod vga;

use vga::VgaColor;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // short example to demonstrate how a shell would look.
    // no actual commands are run, obviously.
    vga::clear(VgaColor::White, VgaColor::Blue);
    let cmds = [
        "cd ~",
        "ls -a",
        "echo hello world",
        "mnt /dev/sda0 /mnt",
        "sudo rm -rf /",
        "emacs -nw test.rs",
        "sudo pacman -Rns vim",
        "neofetch",
        "rustc --version",
        "cargo test && cargo run",
    ];
    loop {
        for cmd in cmds {
            vga::print_str("example@test$ ", VgaColor::White, VgaColor::Blue);
            kcore::waste_cycles(900000);
            vga::print_str(cmd, VgaColor::White, VgaColor::Blue);
            kcore::waste_cycles(900000);
            vga::print_char(b'\n', VgaColor::White, VgaColor::Blue);
        }
    }
}
