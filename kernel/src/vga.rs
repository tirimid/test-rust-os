// interaction with the actual vga buffer.
// this module does not handle any advanced functionality.

use volatile::Volatile;
use crate::memutil;
use core::ptr;

#[allow(dead_code)]
struct VgaChar {
    ch: u8,
    attr: u8,
}

#[allow(dead_code)]
#[derive(Clone, Copy)]
pub enum VgaColor {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Purple = 5,
    Brown = 6,
    Gray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    LightPurple = 13,
    Yellow = 14,
    White = 15,
}

fn vga_attr(fg_col: VgaColor, bg_col: VgaColor) -> u8 {
    (bg_col as u8) << 4 | fg_col as u8
}

const VGA_BUFFER: *mut Volatile<VgaChar> = 0xb8000 as *mut Volatile<VgaChar>;
const VGA_BUFFER_SIZE: (isize, isize) = (80, 25);
const VGA_TAB_SIZE: isize = 8; // in characters.

static mut CURSOR: isize = 0;

// after scrolling, empty area will be filled with specified colors.
// data is lost upon scroll.
pub fn scroll_down(fg_col: VgaColor, bg_col: VgaColor) {
    let attr = vga_attr(fg_col, bg_col) as u64;
    let q = attr << 8 | attr << 24 | attr << 40 | attr << 56;
    unsafe {
        ptr::copy(
            VGA_BUFFER.offset(VGA_BUFFER_SIZE.0),
            VGA_BUFFER,
            (VGA_BUFFER_SIZE.0 * (VGA_BUFFER_SIZE.1 - 1)).try_into().unwrap(),
        );
        memutil::fast_write_mem(
            VGA_BUFFER.offset(VGA_BUFFER_SIZE.0 * (VGA_BUFFER_SIZE.1 - 1)),
            20,
            q,
        );
    }
}

// same things apply as `scroll_down()`.
pub fn scroll_up(fg_col: VgaColor, bg_col: VgaColor) {
    let attr = vga_attr(fg_col, bg_col) as u64;
    let q = attr << 8 | attr << 24 | attr << 40 | attr << 56;
    unsafe {
        ptr::copy(
            VGA_BUFFER,
            VGA_BUFFER.offset(VGA_BUFFER_SIZE.0),
            (VGA_BUFFER_SIZE.0 * (VGA_BUFFER_SIZE.1 - 1)).try_into().unwrap(),
        );
    }
    memutil::fast_write_mem(VGA_BUFFER, 20, q);
}

pub fn print_char(ch: u8, fg_col: VgaColor, bg_col: VgaColor) {
    unsafe {
        match ch {
            b'\t' => CURSOR += VGA_TAB_SIZE,
            b'\n' => CURSOR += VGA_BUFFER_SIZE.0 - CURSOR % VGA_BUFFER_SIZE.0,
            ch => {
                let vga_ch = Volatile::new(VgaChar { ch, attr: vga_attr(fg_col, bg_col) });
                VGA_BUFFER.offset(CURSOR).write(vga_ch);
                CURSOR += 1;
            }
        }
        if CURSOR >= VGA_BUFFER_SIZE.0 * VGA_BUFFER_SIZE.1 {
            CURSOR -= VGA_BUFFER_SIZE.0;
            scroll_down(fg_col, bg_col);
        }
    }
}

// utf-8 characters are read as byte sequence.
pub fn print_str(str_: &str, fg_col: VgaColor, bg_col: VgaColor) {
    for ch in str_.bytes() {
        print_char(ch, fg_col, bg_col);
    }
}

pub fn clear(fg_col: VgaColor, bg_col: VgaColor) {
    let attr = vga_attr(fg_col, bg_col) as u64;
    let q = attr << 8 | attr << 24 | attr << 40 | attr << 56;
    memutil::fast_write_mem(VGA_BUFFER, 0x1f4, q);
    unsafe {
        CURSOR = 0;
    }
}
