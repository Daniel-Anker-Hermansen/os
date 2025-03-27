use core::{arch::asm, hint::unreachable_unchecked};

pub fn cli() {
	unsafe {
		asm!("cli");
	}
}

pub fn sti() {
	unsafe {
		asm!("sti");
	}
}

pub fn hlt() -> ! {
	unsafe {
		asm!("hlt");
		unreachable_unchecked();
	}
}
