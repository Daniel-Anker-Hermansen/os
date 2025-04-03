#![no_std]
#![no_main]
#![allow(named_asm_labels)]
#![feature(sync_unsafe_cell)]

use core::alloc::Layout;

use alloc::alloc::alloc;
use timer::get_time_millis;
use video::draw_ascii_string;
use x86::InteruptEntry;

extern crate alloc;

mod allocator;
mod char;
mod interupt;
mod kbc;
mod panic;
mod pic;
mod timer;
mod video;
mod x86;

#[unsafe(link_section = ".entry")]
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
	pic::remap_pic();
	kbc::initialize_kbc();
	let hello_world = b"HELLO, WORLD!";
	draw_ascii_string(0, 0, hello_world);
	let idt = unsafe { alloc(Layout::array::<InteruptEntry>(256).unwrap()) } as *mut InteruptEntry;
	for i in 0..=255 {
		unsafe {
			x86::set_interupt(idt, i, interupt::IRQ_DEFAULT);
		}
	}
	unsafe {
		x86::set_interupt(idt, 32, interupt::IRQ_32);
	}
	unsafe {
		x86::set_interupt(idt, 33, interupt::IRQ_33);
	}
	x86::lidt(idt);
	x86::sti();
	loop {
		let binary = alloc::format!("{:032b}", get_time_millis());
		draw_ascii_string(0, 18, binary.as_bytes());
		x86::hlt();
	}
}

#[unsafe(no_mangle)]
///# Safety
///`src` and `dest` must point to valid segments of memory of at least length `n`
unsafe extern "C" fn memcpy(dest: *mut u8, src: *const u8, n: usize) -> *mut u8 {
	unsafe {
		for i in 0..n {
			dest.add(i).write(src.add(i).read());
		}
	}
	dest
}

#[unsafe(no_mangle)]
///# Safety
///`dest` must point to a valid segment of memory of at least length `n`
unsafe extern "C" fn memset(dest: *mut u8, content: u8, size: usize) -> *mut u8 {
	for i in 0..size {
		unsafe { dest.add(i).write(content) }
	}
	dest
}
