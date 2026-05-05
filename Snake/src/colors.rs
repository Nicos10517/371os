mod img;

pub fn colors() {
    // Calvin has about 9 lines of code here
    // 1 line was in an unsafe block (which was 3 lines total)
    let vga = 0xb8000 as *mut u8;
    for i in 0..25 {
        for j in 0..80 {

            let color: u8 = (j /5) as u8;

            let pointer = ((i * 80) + j) * 2;
            unsafe {             
                vga.add(pointer).write_volatile(0xDB);
                vga.add(pointer + 1).write_volatile((color <<4) | color);
            }
        }
    }
}

pub fn image() {
    let vga = 0xb8000 as *mut u8;
    for i in 0..25 {
        for j in 0..80 {

            let color= img::CAT_DATA[i * 80 + j];

            let pointer = ((i * 80) + j) * 2;

            unsafe {
                vga.add(pointer).write_volatile(0xDB);
                vga.add(pointer + 1).write_volatile((color << 4 ) | color);
            }
        }
    }
}
