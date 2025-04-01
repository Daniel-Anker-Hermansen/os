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
mod panic;
mod timer;
mod video;
#[allow(unused)]
mod x86;

#[unsafe(link_section = ".entry")]
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
	x86::remap_pic();
	let hello_world = b"HELLO, WORLD!";
	draw_ascii_string(0, 0, hello_world);
	let idt = unsafe { alloc(Layout::array::<InteruptEntry>(256).unwrap()) } as *mut InteruptEntry;
	for i in 0..256 {
		unsafe {
			idt.add(i).write(InteruptEntry::new(x86::DEFALT_INTERUPT));
		}
	}
	unsafe {
		idt.add(32).write(InteruptEntry::new(x86::IRQ_32));
	}
	x86::lidt(idt);
	x86::sti();
	let mut replies = alloc::vec::Vec::new();
	for i in 0usize.. {
		let binary = alloc::format!("{:032b}", get_time_millis());
		draw_ascii_string(0, 18, binary.as_bytes());
		let scan_code = x86::r#in::<0x60>();
		if scan_code != 0 && scan_code != 0xCF {
			replies.push(scan_code);
			let binary = alloc::format!("{:08b}", replies[(i / 100) % replies.len()]); 
			draw_ascii_string(0, 27, binary.as_bytes());
		}
		x86::hlt();
	}
	unreachable!();
}

#[unsafe(no_mangle)]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub extern "C" fn memcpy(dest: *mut u8, src: *const u8, n: usize) -> *mut u8 {
	unsafe {
		for i in 0..n {
			dest.add(i).write(src.add(i).read());
		}
	}
	dest
}

#[unsafe(no_mangle)]
extern "C" fn memset(dest: *mut u8, content: u8, size: usize) -> *mut u8 {
	for i in 0..size {
		unsafe { dest.add(i).write(content) }
	}
	dest
}
