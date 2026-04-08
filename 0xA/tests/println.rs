#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(osirs::_test_runner)]
#![reexport_test_harness_main = "test_main"]

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    osirs::test_panic_handler(info);
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    test_main();
    loop {}
}

const MMIO: usize = 0xb8000;
const COLS: usize = 80;
const COLOR: u8 = 0xF;

fn vga_char_at(row: usize, col: usize) -> u8 {
    let offset = (row * COLS + col) * 2;
    unsafe { *((MMIO + offset) as *const u8) }
}

fn vga_color_at(row: usize, col: usize) -> u8 {
    let offset = (row * COLS + col) * 2 + 1;
    unsafe { *((MMIO + offset) as *const u8) }
}

#[test_case]
fn test_print_appears() {
    osirs::println!("A");
    assert_eq!(vga_char_at(0, 0), b'A');
}

#[test_case]
fn test_scroll_and_color() {
    for i in 0..26usize {
        osirs::println!("{:080x}", i);
    }
    for row in 0..25 {
        for col in 0..COLS {
            assert_eq!(vga_color_at(row, col), COLOR);
        }
    }
}
