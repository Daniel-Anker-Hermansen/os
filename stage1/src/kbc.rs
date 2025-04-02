use crate::x86;

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

}
