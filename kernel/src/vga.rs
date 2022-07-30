// interaction with the actual vga buffer.
// this module does not handle any advanced functionality.

use volatile::Volatile;
use crate::memutil;
use core::ptr;
use crate::cpuio;
use cpuio::IoPort;
use crate::mathutil;

#[allow(dead_code)]
#[repr(C)]
struct VgaChar {
    byte: u8,
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

fn extend_attr(attr: u8) -> u64 {
    let attr = attr as u64;
    attr << 8 | attr << 24 | attr << 40 | attr << 56
}

const VGA_BUFFER: *mut Volatile<VgaChar> = 0xb8000 as *mut Volatile<VgaChar>;
const VGA_BUFFER_SIZE: (u16, u16) = (80, 25);
const VGA_TAB_SIZE: u16 = 8; // in vga characters.

static mut CURSOR: u16 = 0;

fn update_vga_cursor() {
    unsafe {
        cpuio::write_port_u8(IoPort::VgaControl, 0xf);
        cpuio::write_port_u8(IoPort::VgaData, (CURSOR & 0xff) as u8);
        cpuio::write_port_u8(IoPort::VgaControl, 0xe);
        cpuio::write_port_u8(IoPort::VgaData, (CURSOR >> 8) as u8);
    }
}

// after scrolling, empty area will be filled with specified colors.
// data is lost upon scroll.
pub fn scroll_down(fg_col: VgaColor, bg_col: VgaColor) {
    unsafe {
        ptr::copy(
            VGA_BUFFER.offset(VGA_BUFFER_SIZE.0 as isize),
            VGA_BUFFER,
            (VGA_BUFFER_SIZE.0 * (VGA_BUFFER_SIZE.1 - 1)).try_into().unwrap(),
        );
        memutil::fast_write_mem(
            VGA_BUFFER.offset(VGA_BUFFER_SIZE.0 as isize * (VGA_BUFFER_SIZE.1 as isize - 1)),
            20,
            extend_attr(vga_attr(fg_col, bg_col)),
        );
        CURSOR = mathutil::clamp(
            CURSOR - VGA_BUFFER_SIZE.0,
            0,
            VGA_BUFFER_SIZE.0 * VGA_BUFFER_SIZE.1 - 1,
        );
    }
    update_vga_cursor();
}

// same things apply as `scroll_down()`.
pub fn scroll_up(fg_col: VgaColor, bg_col: VgaColor) {
    unsafe {
        ptr::copy(
            VGA_BUFFER,
            VGA_BUFFER.offset(VGA_BUFFER_SIZE.0 as isize),
            (VGA_BUFFER_SIZE.0 * (VGA_BUFFER_SIZE.1 - 1)).try_into().unwrap(),
        );
        CURSOR = mathutil::clamp(
            CURSOR + VGA_BUFFER_SIZE.0,
            0,
            VGA_BUFFER_SIZE.0 * VGA_BUFFER_SIZE.1 - 1,
        );
    }
    memutil::fast_write_mem(VGA_BUFFER, 20, extend_attr(vga_attr(fg_col, bg_col)));
    update_vga_cursor();
}

// strictly for internal use.
// only updates vga cursor if scroll occurs, allowing for optimizations in other functions.
fn write_byte(byte: u8, fg_col: VgaColor, bg_col: VgaColor) {
    unsafe {
        match byte {
            b'\t' => {
                let line_offset = CURSOR % VGA_BUFFER_SIZE.0;
                CURSOR -= line_offset % VGA_TAB_SIZE;
                CURSOR += VGA_TAB_SIZE;
            }
            b'\n' => CURSOR += VGA_BUFFER_SIZE.0 - CURSOR % VGA_BUFFER_SIZE.0,
            byte => {
                let vga_ch = Volatile::new(VgaChar { byte, attr: vga_attr(fg_col, bg_col) });
                VGA_BUFFER.offset(CURSOR as isize).write(vga_ch);
                CURSOR += 1;
            }
        }
        if CURSOR >= VGA_BUFFER_SIZE.0 * VGA_BUFFER_SIZE.1 {
            scroll_down(fg_col, bg_col);
        }
    }
}

pub fn print_byte(byte: u8, fg_col: VgaColor, bg_col: VgaColor) {
    write_byte(byte, fg_col, bg_col);
    update_vga_cursor();
}

// utf-8 characters are read as byte sequence.
pub fn print_str(str_: &str, fg_col: VgaColor, bg_col: VgaColor) {
    for byte in str_.bytes() {
        print_byte(byte, fg_col, bg_col);
    }
    update_vga_cursor();
}

pub fn clear(fg_col: VgaColor, bg_col: VgaColor) {
    memutil::fast_write_mem(VGA_BUFFER, 0x1f4, extend_attr(vga_attr(fg_col, bg_col)));
    unsafe {
        CURSOR = 0;
    }
    update_vga_cursor();
}
