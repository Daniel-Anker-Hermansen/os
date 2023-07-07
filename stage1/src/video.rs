pub fn draw_pixel(x: usize, y: usize, color: u8) {
    unsafe {
        (0xA0000 as *mut u8).add(320 * y + x).write(color);
    }
}

pub fn draw_char(x: usize, a: char) {
    let rx = 3 * x;
    for x in rx..rx + 2 {
        for y in 0..a as usize - 48 {
            draw_pixel(x, y, 4); 
        }
    }
}
