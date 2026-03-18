#![no_std]
#![no_main]

mod vga;
mod colors;

#[panic_handler]
#[allow(unconditional_recursion)]
fn panic(info: &core::panic::PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    colors::image();
    loop{}
}
