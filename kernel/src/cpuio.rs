use core::arch::asm;

#[allow(dead_code)]
pub enum IoPort {
    VgaControl = 0x3d4,
    VgaData = 0x3d5,
}

pub fn read_port_u8(port: IoPort) -> u8 {
    let mut data;
    unsafe {
        asm!(
            "in al, dx",
            out("al") data,
            in("dx") port as u16,
        );
    }
    data
}

pub fn read_port_u16(port: IoPort) -> u16 {
    let mut data;
    unsafe {
        asm!(
            "in ax, dx",
            out("ax") data,
            in("dx") port as u16,
        );
    }
    data
}

pub fn read_port_u32(port: IoPort) -> u32 {
    let mut data;
    unsafe {
        asm!(
            "in eax, dx",
            out("eax") data,
            in("dx") port as u16,
        );
    }
    data
}

pub fn write_port_u8(port: IoPort, data: u8) {
    unsafe {
        asm!(
            "out dx, al",
            in("dx") port as u16,
            in("al") data,
        );
    }
}

pub fn write_port_u16(port: IoPort, data: u16) {
    unsafe {
        asm!(
            "out dx, ax",
            in("dx") port as u16,
            in("ax") data,
        );
    }
}

pub fn write_port_u32(port: IoPort, data: u32) {
    unsafe {
        asm!(
            "out dx, eax",
            in("dx") port as u16,
            in("eax") data,
        );
    }
}
