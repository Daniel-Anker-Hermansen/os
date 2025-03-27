#![no_std]
#![no_main]
#![allow(named_asm_labels)]

use video::draw_ascii_string;

extern crate alloc;

mod allocator;
mod char;
mod panic;
mod video;
#[allow(unused)]
mod x86;

#[link_section = ".entry"]
#[no_mangle]
pub extern "C" fn _start() -> ! {
	let hello_world = b"HELLO, WORLD!";
	draw_ascii_string(0, 0, hello_world);
	x86::hlt();
}

#[no_mangle]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
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
