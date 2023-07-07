#![no_std]
#![no_main]

mod video;

use core::{panic::PanicInfo, arch::{asm, global_asm}};

use video::{write_pixel, write_number};

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    write_pixel(2, 2, 4);
    loop { }
}

#[repr(C, packed)]
struct IDTDescriptor {
    size: u16,
    offset: u32,
}

static IDT: IDTDescriptor = IDTDescriptor {
    size: 256 * 8 - 1,
    offset: 0x00,
};

#[link_section = ".entry"]
#[no_mangle]
pub extern fn _start() {
    create_idt();
    for i in 0..256 {
        register_idt(i, int21);
    }
    sti();
    write_number(int21 as usize, 0, 0);
}

fn create_idt() {
    unsafe {
        asm!(
            "lidt [{r0}]",
            r0 = in(reg) &IDT
        )
    }
}

#[repr(C, packed)]
struct InteruptHandler {
    offset_l: u16,
    segment: u16,
    reserved: u8,
    flags: u8,
    offset_h: u16,
}


fn register_idt(int: usize, handler: unsafe extern fn()) {
    let offset = handler as usize;
    let offset_h = (offset & 0xFFFF) as u16;
    let offset_l = (offset >> 16) as u16;
    let handler = InteruptHandler {
        offset_l,
        segment: 0x10,
        reserved: 0,
        flags: 0x8E,
        offset_h,
    };
    unsafe { 
        (0x00 as *mut InteruptHandler)
            .add(int)
            .write_volatile(handler); 
    }
}


fn cli() {
    unsafe {
        asm!{
            "cli"
        }
    }
}

fn sti() {
    unsafe {
        asm!{
            "sti"
        }
    }
}

#[no_mangle]
extern fn r_int21() {
    write_pixel(0, 0, 2);
    write_pixel(1, 1, 2);
}

global_asm!{
"int21:",
    "pushad",
    "call r_int21",
    "popad",
    "iret",
}

extern {
    fn int21();
}
