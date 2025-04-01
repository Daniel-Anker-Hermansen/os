use crate::char::CHAR_BIT_MAP;

pub fn draw_pixel(x: usize, y: usize, color: u8) {
	unsafe {
		(0xA0000 as *mut u8).add(320 * y + x).write(color);
	}
}

pub fn draw_ascii(x: usize, y: usize, a: u8) {
	for rx in 0..5 {
		for ry in 0..8 {
			let color = if CHAR_BIT_MAP[a as usize][ry] & (1 << (4 - rx)) != 0 {
				2
			} else {
				0
			};
			draw_pixel(x + rx, y + ry, color);
		}
	}
}

pub fn draw_ascii_string(x: usize, y: usize, s: &[u8]) {
	for (offset, a) in s.iter().copied().enumerate() {
		draw_ascii(x + offset * 6, y, a);
	}
}
