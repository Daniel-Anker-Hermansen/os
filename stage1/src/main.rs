#![no_std]
#![no_main]
#![allow(named_asm_labels)]

use alloc::vec::Vec;

extern crate alloc;

mod allocator;
mod video;
mod panic;
//mod vec;

fn rdrand32() -> u32 {
    5
}

fn __sti() { 
    unsafe {
        core::arch::asm!("sti");
    }
}

#[link_section = ".entry"]
#[no_mangle]
pub extern "C" fn _start() {
    // sti();
    let mut v = Vec::new();
    for _ in 0..12 {
        v.push(rdrand32() % 10);
        unsafe {
            core::arch::asm!(
                "nop",
                in("eax") &v,
            );
        }
    }
    for (i, ch) in (0..12).map(|i| &v[i]).cloned().enumerate() {
        video::draw_char(i, (ch as u8 + 48) as char);
    }
    loop { }
}

#[no_mangle]
pub extern "C" fn memcpy(dest: *mut u8, src: *const u8, n: usize) -> *mut u8 {
    unsafe {
        for i in 0..n {
            dest.add(i).write(src.add(i).read());
        }
    }
    dest
}

#[no_mangle]
extern "C" fn memset(dest: *mut u8, content: u8, size: usize) -> *mut u8 {
    for i in 0..size {
        unsafe { dest.add(i).write(content) }
    }
    dest
}
