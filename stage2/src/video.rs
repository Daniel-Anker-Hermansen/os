const VRAM_PTR: *mut u8 = 0xA0000 as _;


pub fn write_pixel(x: usize, y: usize, color: u8) {
    unsafe {
        VRAM_PTR.add(y * 320 + x).write_volatile(color);
    }
}

const NUMERIC_FONT: [[u8; 4]; 10] = [
    [0x7E, 0x81, 0x81, 0x7E],
    [0x00, 0x00, 0xFF, 0x00],
    [0x43, 0x8D, 0xB1, 0x41],
    [0x81, 0x89, 0x89, 0x76],
    [0xF8, 0x08, 0x08, 0xFF],
    [0x71, 0x89, 0x89, 0x86],
    [0x7E, 0x89, 0x89, 0x86],
    [0x83, 0x8C, 0xB0, 0xC0],
    [0x76, 0x89, 0x89, 0x76],
    [0x71, 0x89, 0x89, 0x7E],
];

pub fn write_number(number: usize, x: usize, y: usize) {
    write_nummber_rec(number, x, y);
}

fn write_nummber_rec(number: usize, x: usize, y: usize) -> usize {
    if number < 10 {
        write_digit(number, x, y);
        1
    }
    else {
        let offset = write_nummber_rec(number / 10, x, y);
        write_digit(number % 10, x + 5 * offset, y);
        offset + 1
    }
}

// Inlining causes a bug where arguments are not passed correctly. I do not understand why that is
// the case.
#[inline(always)]
pub fn write_digit(digit: usize, x: usize, y: usize) {
    let font = NUMERIC_FONT[digit];
    for xd in x..x + 4 {
        let mut data = font[xd - x];
        for yd in (y..y + 8).rev() {
            write_pixel(xd, yd, 2 * (data & 1));
            data >>= 1;
        }
    }
}
