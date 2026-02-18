#![no_std]
#![no_main]

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    
    let vga = 0xb8000 as *mut u8;

    let ints: [i32; 3] = [1819043144, 1870078063, 560229490];

    let pointer = &ints as *const i32 as *const u8;


    for i in 0..12 {
        unsafe{
            *((vga as usize + 2 * i) as *mut u8) = *((pointer as usize + i) as *const u8);
            *((vga as usize + 2 * i + 1) as *mut u8) = 0xF;
        }
    }

    loop {}
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
