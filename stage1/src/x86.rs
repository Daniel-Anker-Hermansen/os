use core::{
	arch::{asm, global_asm},
	hint::unreachable_unchecked,
};

use crate::video::draw_ascii_string;

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
			offset_1: ((ptr as usize) >> 0) as u16,
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

#[unsafe(no_mangle)]
extern "C" fn write_ehlo() {
	draw_ascii_string(0, 9, b"EHLO");
	loop {}
}

#[unsafe(no_mangle)]
extern "C" fn write_leho() {
	draw_ascii_string(0, 18, b"LEHO");
}

global_asm!(
	"
.global interupt_handler
__irq_32:
	   push rax
	   push rbx
	   push rcx
	   push rdx
	   push rdi
	   push rsi
	   push rbp
	   push r8
	   push r9
	   push r10
	   push r11
	   push r12
	   push r13
	   push r14
	   push r15
	   cld
	   call pit_interupt_handler
	   pop r15
	   pop r14
	   pop r13
	   pop r12
	   pop r11
	   pop r10
	   pop r9
	   pop r8
	   pop rbp
	   pop rsi
	   pop rsi
	   pop rdx
	   pop rcx
	   pop rbx
	   pop rax
	iretq
.global interupt_handler
__interupt_handler:
	   push rax
	   push rbx
	   push rcx
	   push rdx
	   push rdi
	   push rsi
	   push rbp
	   push r8
	   push r9
	   push r10
	   push r11
	   push r12
	   push r13
	   push r14
	   push r15
	   cld
	   call write_ehlo
	   call pic1_eoi
	   pop r15
	   pop r14
	   pop r13
	   pop r12
	   pop r11
	   pop r10
	   pop r9
	   pop r8
	   pop rbp
	   pop rsi
	   pop rsi
	   pop rdx
	   pop rcx
	   pop rbx
	   pop rax
	iretq
.global exception_handler
exception_handler:
	", /*
	   push rax
	   push rbx
	   push rcx
	   push rdx
	   push rdi
	   push rsi
	   push rbp
	   push r8
	   push r9
	   push r10
	   push r11
	   push r12
	   push r13
	   push r14
	   push r15
	   cld
	   call write_leho
	   pop r15
	   pop r14
	   pop r13
	   pop r12
	   pop r11
	   pop r10
	   pop r9
	   pop r8
	   pop rbp
	   pop rsi
	   pop rsi
	   pop rdx
	   pop rcx
	   pop rbx
	   pop rax
	   add rsp, 8
	   */
	"
	iretq
"
);

unsafe extern "C" {
	pub static __interupt_handler: u8;

	pub static __irq_32: u8;
}

pub const IRQ_32: *const u8 = unsafe { &__irq_32 };
pub const DEFALT_INTERUPT: *const u8 = unsafe { &__interupt_handler };

pub fn out<const PORT: u16>(value: u8) {
	unsafe {
		asm!("out {}, al", const PORT, inout("al") value => _);
	}
}

pub fn r#in<const PORT: u16>() -> u8 {
	let value;
	unsafe {
		asm!("in al, {}", const PORT, out("al") value);
	}
	value
}

pub fn io_wait() {
	out::<0x80>(0);
}

pub const PIC1_COMMAND: u16 = 0x20;
pub const PIC1_DATA: u16 = 0x21;
pub const PIC2_COMMAND: u16 = 0xA0;
pub const PIC2_DATA: u16 = 0xA1;

pub fn remap_pic() {
	out::<PIC1_COMMAND>(0x11);
	io_wait();
	out::<PIC2_COMMAND>(0x11);
	io_wait();
	out::<PIC1_DATA>(0x20);
	io_wait();
	out::<PIC2_DATA>(0x28);
	io_wait();
	out::<PIC1_DATA>(4);
	io_wait();
	out::<PIC2_DATA>(2);
	io_wait();
	out::<PIC1_DATA>(0x01);
	io_wait();
	out::<PIC2_DATA>(0x01);
	io_wait();
	out::<PIC1_DATA>(0x00);
	io_wait();
	out::<PIC2_DATA>(0x00);
	io_wait();
}

#[unsafe(no_mangle)]
pub extern "C" fn pic1_eoi() {
	out::<PIC1_COMMAND>(0x20);
	io_wait();
}

pub fn write_read_pc2(command: u8) -> u8 {
	out::<0x64>(command);
	while r#in::<0x64>() & 0x01 == 0 {}
	r#in::<0x60>()
}
