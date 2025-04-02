use crate::x86;

pub const PIC1_COMMAND: u8 = 0x20;
pub const PIC1_DATA: u8 = 0x21;
pub const PIC2_COMMAND: u8 = 0xA0;
pub const PIC2_DATA: u8 = 0xA1;

fn command_1(command: u8) {
	x86::out::<PIC1_COMMAND>(command);
}

fn command_2(command: u8) {
	x86::out::<PIC2_COMMAND>(command);
}

fn write_data_1(data: u8) {
	x86::out::<PIC1_DATA>(data);
}

fn write_data_2(data: u8) {
	x86::out::<PIC2_DATA>(data);
}

fn wait() {
	x86::out::<0x80>(0x00);
}

pub fn remap_pic() {
	command_1(0x11);
	command_2(0x11);
	wait();

	write_data_1(0x20);
	write_data_2(0x28);
	wait();

	write_data_1(4);
	write_data_2(2);
	wait();

	write_data_1(0x01);
	write_data_2(0x01);
	wait();
	
	write_data_1(0x00);
	write_data_2(0x00);
	wait();
}

pub fn eoi1() {
	command_1(0x20);
	wait();
}
