#![no_std]
#![no_main]
#![allow(named_asm_labels)]

#[cfg(not(test))]
use core::panic::PanicInfo;

use alloc::string::ToString;

#[cfg(not(test))]
#[panic_handler]
fn panic_handler(_: &PanicInfo) -> ! {
    loop { }
}


extern crate alloc;

mod allocator;
mod video;

#[link_section = ".entry"]
#[no_mangle]
pub extern fn _start() {
    video::draw_pixel(0, 0, 4);
    let s = 123u8.to_string();
    for (i, ch) in s.chars().enumerate() {
        video::draw_char(i, ch);
    }
    loop { }
}

#[no_mangle]
pub extern fn memcpy(dest: *mut u8, src: *const u8, n: usize) -> *mut u8 {
    unsafe {
        for i in 0..n {
            dest.add(i).write(src.add(i).read());
        }
    }
    dest
}

