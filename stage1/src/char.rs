const EMPTY: [u8; 8] = [0; 8];

const fn generate_char_bit_map() -> [[u8; 8]; 128] {
	let mut char_bit_map = [EMPTY; 128];
	char_bit_map[b'D' as usize] = [
		0b11110,
		0b11111,
		0b11011,
		0b11011,
		0b11011,
		0b11011,
		0b11111,
		0b11110,
	];
	char_bit_map[b'E' as usize] = [
		0b11111,
		0b11111,
		0b11000,
		0b11111,
		0b11111,
		0b11000,
		0b11111,
		0b11111,
	];
	char_bit_map[b'H' as usize] = [
		0b11011,
		0b11011,
		0b11011,
		0b11111,
		0b11111,
		0b11011,
		0b11011,
		0b11011,
	];
	char_bit_map[b'L' as usize] = [
		0b11000,
		0b11000,
		0b11000,
		0b11000,
		0b11000,
		0b11000,
		0b11111,
		0b11111,
	];
	char_bit_map[b'O' as usize] = [
		0b11111,
		0b11111,
		0b11011,
		0b11011,
		0b11011,
		0b11011,
		0b11111,
		0b11111,
	];
	char_bit_map[b'R' as usize] = [
		0b11110,
		0b11111,
		0b11011,
		0b11011,
		0b11110,
		0b11110,
		0b11011,
		0b11011,
	];
	char_bit_map[b'W' as usize] = [
		0b10101,
		0b10101,
		0b10101,
		0b11111,
		0b11111,
		0b11111,
		0b01010,
		0b01010,
	];
	char_bit_map[b'!' as usize] = [
		0b01100,
		0b01100,
		0b01100,
		0b01100,
		0b00000,
		0b00000,
		0b01100,
		0b01100,
	];
	char_bit_map
}

pub const CHAR_BIT_MAP: [[u8; 8]; 128] = generate_char_bit_map(); 
