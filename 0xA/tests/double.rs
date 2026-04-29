#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(osirs::_test_runner)]
#![reexport_test_harness_main = "test_main"]

#[panic_handler]
fn test_panic(_info: &core::panic::PanicInfo) -> ! {
    osirs::serial_println!("{}", _info);
    osirs::serial_println!("[Pass]");
    osirs::qemu_quit(osirs::QEMU_PASS);
}


fn bad() {
    unsafe { *(0xdeadbeef as *mut u8) = 42; };
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    osirs::init();
    osirs::_test_runner(&[&bad]);
    osirs::qemu_quit(osirs::QEMU_FAIL);
}
