use crate::{pic, video::draw_ascii_string, x86};

const DATA: u8 = 0x60;
const COMMAND: u8 = 0x64;

fn read_data() -> u8 {
	while x86::r#in::<COMMAND>() & 0x01 == 0 {}
	x86::r#in::<DATA>()
}

fn write_data(data: u8) {
	while x86::r#in::<COMMAND>() & 0x02 != 0 {}
	x86::out::<DATA>(data);
}

fn command(cmd: u8) {
	x86::out::<COMMAND>(cmd);
}

fn command_response(cmd: u8) -> u8 {
	command(cmd);
	read_data()
}

fn command_data(cmd: u8, data: u8) {
	command(cmd);
	write_data(data);
}

pub fn initialize_kbc() {
	// Disable devices
	command(0xAD);
	command(0xA7);

	// Flush
	x86::r#in::<DATA>();

	// Set controller configuration byte
	let config = command_response(0x20);
	command_data(0x60, config & 0b10101110);

	// Self-test
	let status = command_response(0xAA);
	assert_eq!(status, 0x55);

	// PS2-channel 1 test
	let status = command_response(0xAB);
	assert_eq!(status, 0x00);

	// Enable
	command(0xAE);

	// Reset device
	write_data(0xFF);
	let a = read_data();
	let b = read_data();
	assert!((a == 0xFA && b == 0xAA) || (a == 0xAA && b == 0xFA));
	
	write_data(0xF4);
	let response = read_data();
	assert_eq!(response, 0xFA);

	// Set interupts
	let config = command_response(0x20);
	command_data(0x60, config | 1);
}

#[unsafe(no_mangle)]
extern "C" fn keyboard_input() {
	let scan = x86::r#in::<DATA>();
	draw_ascii_string(0, 36, alloc::format!("{:08b}", scan).as_bytes());
	pic::eoi1();
}
