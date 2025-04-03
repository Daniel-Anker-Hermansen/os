use crate::{pic, video::draw_ascii_string};


macro_rules! interupt {
	($name:ident, $asm_name:ident, $f:ident) => {
		core::arch::global_asm!(
			concat!(stringify!($asm_name), ":"),
			"push rax
			push rcx
			push rdx
			push rdi
			push rsi
			push r8
			push r9
			push r10
			push r11
			cld",
			concat!("call ", stringify!($f)),
			"pop r11
			pop r10
			pop r9
			pop r8
			pop rsi
			pop rdi
			pop rdx
			pop rcx
			pop rax
			iretq",
		);
		unsafe extern "C" {
			static $asm_name: u8;
		}
		pub const $name: *const u8 = unsafe { &$asm_name };
	};
}

interupt!(IRQ_32, __irq_32, pit_interupt_handler);
interupt!(IRQ_33, __irq_33, keyboard_input);
interupt!(IRQ_DEFAULT, __irq_default, default_handler);

#[unsafe(no_mangle)]
extern "C" fn default_handler() {
	write_ehlo();
	pic::eoi1();
}

fn write_ehlo() {
	draw_ascii_string(0, 9, b"EHLO");
}
