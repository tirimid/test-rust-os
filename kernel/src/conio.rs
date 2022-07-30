use crate::vga;
use vga::VgaColor;
use crate::memutil;
use core::str;
use core::fmt;

pub trait ConBuffer {
    fn write_byte(&mut self, byte: u8);
    fn flush(&mut self);
}

pub struct OutBuffer {
    buf: [u8; 512],
    cursor: usize,
    pub flush_fg_col: VgaColor,
    pub flush_bg_col: VgaColor,
}

impl ConBuffer for OutBuffer {
    fn write_byte(&mut self, byte: u8) {
        self.buf[self.cursor] = byte;
        self.cursor += 1;
    }
    
    fn flush(&mut self) {
        vga::print_str(
            str::from_utf8(&self.buf[..self.cursor]).unwrap(),
            self.flush_fg_col,
            self.flush_bg_col,
        );
        memutil::fast_write_mem(self.buf.as_ptr(), (self.buf.len() / 8).try_into().unwrap(), 0);
        self.cursor = 0;
    }
}

impl fmt::Write for OutBuffer {
    fn write_str(&mut self, str_: &str) -> fmt::Result {
        for byte in str_.bytes() {
            self.write_byte(byte);
        }
        Ok(())
    }
}

pub static mut CONOUT: OutBuffer = OutBuffer {
    buf: [0; 512],
    cursor: 0,
    flush_fg_col: VgaColor::LightGreen,
    flush_bg_col: VgaColor::Black,
};

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ({
        use core::fmt::Write;
        use crate::conio;
        use crate::conio::ConBuffer;
        
        unsafe {
            conio::CONOUT.write_fmt(format_args!($($arg)*)).unwrap();
            conio::CONOUT.flush();
        }
    });
}

#[macro_export]
macro_rules! println {
    () => (print!("\n"));
    ($($arg:tt)*) => (print!("{}\n", format_args!($($arg)*)));
}
