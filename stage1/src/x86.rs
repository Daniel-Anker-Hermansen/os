use core::arch::asm;

/*
pub fn cli() {
	unsafe {
		asm!("cli");
	}
}
*/

pub fn sti() {
	unsafe {
		asm!("sti");
	}
}

pub fn hlt() {
	unsafe {
		asm!("hlt");
	}
}

// Static memory for writing the IDT data when loading the idt register
static mut IDT: [u8; 10] = [0; 10];

#[repr(C)]
pub struct InteruptEntry {
	offset_1: u16,
	selector: u16,
	interupt_stack_table: u8,
	type_attributes: u8,
	offset_2: u16,
	offset_3: u32,
	reserved: u32,
}

impl InteruptEntry {
	pub fn new(ptr: *const u8) -> InteruptEntry {
		InteruptEntry {
			offset_1: (ptr as usize) as u16,
			selector: 0x08,
			interupt_stack_table: 0,
			type_attributes: 0x8e,
			offset_2: ((ptr as usize) >> 16) as u16,
			offset_3: ((ptr as usize) >> 32) as u32,
			reserved: 0,
		}
	}
}

pub fn lidt(ptr: *mut InteruptEntry) {
	unsafe {
		asm!(
			"mov word ptr [{idt}], 4096",
			"mov qword ptr [{idt} + 2], {}",
			"lidt [{idt}]",
			in(reg) ptr,
			idt = sym IDT,
		);
	}
}

pub fn out<const PORT: u8>(value: u8) {
	unsafe {
		asm!("out {}, al", const PORT, inout("al") value => _);
	}
}

pub fn r#in<const PORT: u8>() -> u8 {
	let value;
	unsafe {
		asm!("in al, {}", const PORT, out("al") value);
	}
	value
}

///# Safety
///`idt` has to be the interupt descripter table pointer and `handler`has to be a valid interupt handler.
pub unsafe fn set_interupt(idt: *mut InteruptEntry, index: u8, handler: *const u8) {
	unsafe {
		idt.add(index as _).write(InteruptEntry::new(handler));
	}
}
