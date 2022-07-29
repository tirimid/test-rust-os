use core::arch::asm;

// uses string operations to optimize memset-like writing.
// only operates on 8-byte size chunks.
pub fn fast_write_mem<T>(dst: *const T, write_cnt: u64, q: u64) {
    unsafe {
        asm!(
            "mov rdi, {dst}",
            "mov rcx, {write_cnt}",
            "mov rax, {q}",
            "rep stosq",
            dst = in(reg) dst,
            write_cnt = in(reg) write_cnt,
            q = in(reg) q,
        );
    }
}
