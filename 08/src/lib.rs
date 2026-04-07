#![no_main]
#![no_std]
#![feature(custom_test_frameworks)]
#![test_runner(_test_runner)]

pub mod vga;
pub mod serial;

pub const QEMU_PASS: u32 = 0xA;
pub const QEMU_FAIL: u32 = 0xB;

pub fn qemu_quit(code: u32) -> ! {
    unsafe {
        x86_64::instructions::port::Port::new(0xf4).write(code);
    }
    loop{}
}

pub fn test_panic_handler(info: &core::panic::PanicInfo) -> ! {
    serial_println!("[Fail]");
    serial_println!("{}", info);

    qemu_quit(QEMU_FAIL);
}


pub fn _test_runner(tests: &[&dyn Fn()]) {
    for (i, test) in tests.iter().enumerate() {
        serial_print!("Beginning test 0x{:02x}...", i);
        test();
        serial_println!(" [Pass]");
    }

    qemu_quit(QEMU_PASS);
}

#[cfg(test)]
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    qemu_quit(QEMU_PASS);
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    test_panic_handler(info);
}
