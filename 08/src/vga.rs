static mut LATEST: usize = 0;
const MMIO: usize = 0xb8000;
const COLOR: u8 = 0xF;

fn char_to_vga(a: u8) {
    unsafe {
        let rel: *mut u8 = (MMIO + LATEST * 2) as *mut u8;
        *rel = a;
        *((rel as usize + 1) as *mut u8) = COLOR;
        LATEST = LATEST + 1;
    }
    return;
}

const ROWS: usize = 80;
const COLS: usize = 25;
const MAX: usize = ROWS * COLS;
const SPACE: u8 = 32;
const ENTER: u8 = 10;

fn scroll() {
    unsafe {
        for i in ROWS..=MAX {
            let src: *mut u8 = (MMIO  + i * 2) as *mut u8;
            let dst: *mut u8 = (MMIO + (i - ROWS) * 2) as *mut u8;
            *dst = *src;
            *((dst as usize + 1) as *mut u8) = COLOR;
        }
        for i in 1..=ROWS {
            let dst: *mut u8 = (MMIO + (MAX + i - ROWS) * 2) as *mut u8;
            *dst = SPACE;
            *((dst as usize + 1) as *mut u8) = COLOR;
        }
        LATEST = LATEST - ROWS;
    }
    return;
}

fn str_to_vga(s: &str) {
    let v = s.as_bytes();
    unsafe {
        for i in 0..v.len() {
            if LATEST > MAX {
                scroll();
            }
            match v[i] {
                ENTER => LATEST = ((LATEST / ROWS) + 1) * ROWS, // newline
                _ => char_to_vga(v[i]),
            }
        }
    }
    return;
}

pub struct Dummy {}

impl core::fmt::Write for Dummy {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        str_to_vga(s);
        return core::result::Result::Ok(());
    }
}

pub fn _print(args: core::fmt::Arguments) {
    use core::fmt::Write;
    let mut d = Dummy {};
    d.write_fmt(args).unwrap();
    return;
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}